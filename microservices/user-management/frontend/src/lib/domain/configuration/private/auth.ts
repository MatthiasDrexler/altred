export class Auth {
  public readonly AUTH_URL: string = process.env["ALTRED_UM_AUTH_URL"]
  public readonly AUTH_CLIENT_ID: string = process.env["ALTRED_UM_AUTH_CLIENT_ID"]
  public readonly AUTH_CLIENT_SECRET: string = process.env["ALTRED_UM_AUTH_CLIENT_SECRET"]
  public readonly AUTH_REDIRECT_URI: string = process.env["ALTRED_UM_AUTH_REDIRECT_URI"]
}

export const auth = new Auth()
