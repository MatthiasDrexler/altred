import { AuthorizationService } from "./authorizationService"
import fetch from "jest-fetch-mock"

describe("AuthorizationService", () => {
  let authorizationService: AuthorizationService

  beforeEach(() => {
    fetch.enableMocks()

    authorizationService = new AuthorizationService()
  })

  it("should invoke token endpoint from keycloak with authorization code", async () => {
    const authorizationCode = "authorization code"
    const accessTokenFromKeycloak = { token_type: "Bearer", access_token: "accesstoken" }
    fetch.mockOnce(JSON.stringify(accessTokenFromKeycloak))

    await authorizationService.retrieveTokenForUser(authorizationCode)

    expect(fetch).toHaveBeenCalledWith(
      process.env["ALTRED_UM_AUTH_URL"],
      expect.objectContaining({
        method: "POST",
        headers: {
          "Content-Type": expect.stringContaining("application/x-www-form-urlencoded"),
        },
        body: expect.any(URLSearchParams),
      })
    )
    const usedUrlSearchParams = fetch.mock.calls[0][1].body as URLSearchParams
    expect(usedUrlSearchParams.get("grant_type")).toBe("authorization_code")
    expect(usedUrlSearchParams.get("client_id")).toBe(process.env["ALTRED_UM_AUTH_CLIENT_ID"])
    expect(usedUrlSearchParams.get("client_secret")).toBe(process.env["ALTRED_UM_AUTH_CLIENT_SECRET"])
    expect(usedUrlSearchParams.get("code")).toBe(authorizationCode)
    expect(usedUrlSearchParams.get("redirect_uri")).toBe(process.env["ALTRED_UM_AUTH_REDIRECT_URI"])
  })
})
