export class Secrets {
  public readonly AUTH_CLIENT_ID = process.env["ALTRED_UM_AUTH_CLIENT_ID"]
  public readonly AUTH_CLIENT_SECRET = process.env["ALTRED_UM_AUTH_CLIENT_SECRET"]
}

export const secrets = new Secrets()
