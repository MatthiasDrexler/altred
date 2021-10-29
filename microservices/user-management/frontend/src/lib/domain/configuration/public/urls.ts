export class Urls {
  public readonly BASE_URL = process.env["ALTRED_UM_FRONTEND_BASE_URL"]
  public readonly LOGIN_URL = process.env["ALTRED_UM_AUTH_LOGIN_URL"]
}

export const urls = new Urls()
