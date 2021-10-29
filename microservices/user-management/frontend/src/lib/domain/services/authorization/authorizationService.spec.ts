import { AuthorizationService } from "./authorizationService"
import fetch from "jest-fetch-mock"

describe("AuthorizationService", () => {
  let authorizationService: AuthorizationService

  beforeEach(() => {
    fetch.enableMocks()

    authorizationService = new AuthorizationService()
  })

  it("should retrieve token from keycloak with authorization code", async () => {
    const accessTokenFromKeycloak = { token_type: "Bearer", access_token: "accesstoken" }
    fetch.mockOnce(JSON.stringify(accessTokenFromKeycloak))

    const token = await authorizationService.retrieveTokenForUser("authorization code")

    expect(token.accessToken).toBe(accessTokenFromKeycloak.access_token)
    expect(fetch).toHaveBeenCalledWith(
      process.env.ALTRED_UM_AUTH_URL,
      expect.objectContaining({
        method: "POST",
        headers: {
          "Content-Type": expect.stringContaining("application/x-www-form-urlencoded"),
        },
        body: expect.any(URLSearchParams),
      })
    )
  })
})
