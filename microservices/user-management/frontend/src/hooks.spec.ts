import { dotenvOutput } from "./hooks"

describe("Hooks", () => {
  it("should load environment variables", () => {
    expect(dotenvOutput.error).not.toBeDefined()
    expect(dotenvOutput.parsed).toMatchObject({
      ALTRED_UM_SUITE_NAME: expect.any(String),
      ALTRED_UM_APPLICATION_NAME: expect.any(String),
    })
  })
})
