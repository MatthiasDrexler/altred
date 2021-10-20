export class Names {
  public readonly SUITE_NAME: string = process.env["ALTRED_UM_SUITE_NAME"]
  public readonly APPLICATION_NAME: string = process.env["ALTRED_UM_APPLICATION_NAME"]
}

export const names = new Names()
