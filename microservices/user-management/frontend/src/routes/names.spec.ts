import { get } from "./names"

describe("Names", () => {
  it("should suite name", () => {
    const names = get()

    expect(names.body.SUITE_NAME).toBeDefined()
    expect(names.body.SUITE_NAME).toBe(process.env.ALTRED_UM_SUITE_NAME)
  })

  it("should contain application name", () => {
    const names = get()

    expect(names.body.APPLICATION_NAME).toBeDefined()
    expect(names.body.APPLICATION_NAME).toBe(process.env.ALTRED_UM_APPLICATION_NAME)
  })
})
