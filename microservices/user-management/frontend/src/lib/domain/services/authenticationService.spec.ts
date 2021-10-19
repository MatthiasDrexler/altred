import { authenticationService } from "./authenticationService"

describe("AuthenticationService", () => {
  it("should be initially unauthenticated", () => {
    expect(authenticationService.isAuthenticated()).toBeFalsy()
  })
})
