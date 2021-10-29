import { get } from "./urls"

describe("Urls", () => {
  it("should contain frontend base url", () => {
    const names = get()

    expect(names.body.BASE_URL).toBeDefined()
    expect(names.body.BASE_URL).toBe(process.env["ALTRED_UM_FRONTEND_BASE_URL"])
  })

  it("should contain login url", () => {
    const names = get()

    expect(names.body.LOGIN_URL).toBeDefined()
    expect(names.body.LOGIN_URL).toBe(process.env["ALTRED_UM_AUTH_LOGIN_URL"])
  })
})
