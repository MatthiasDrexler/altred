import { AuthorizationService } from "./authorizationService"

describe("AuthorizationService", () => {
  let authorizationService: AuthorizationService

  const unmockedFetch = global.fetch

  beforeAll(() => {
    global.fetch = () =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ token_type: "Bearer", access_token: "accesstoken" }),
      }) as Promise<Response>
  })

  afterAll(() => {
    global.fetch = unmockedFetch
  })

  beforeEach(() => {
    authorizationService = new AuthorizationService()
  })

  it("should ", async () => {
    const authorizationCode = "authorization code"

    const token = await authorizationService.retrieveTokenForUser(authorizationCode)

    expect(token.accessToken).toBe("accesstoken")
  })
})
