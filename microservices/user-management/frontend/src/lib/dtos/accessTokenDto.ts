export class AccessTokenDto {
  public readonly tokenType: string
  public readonly accessToken: string

  constructor(tokenType: string, accessToken: string) {
    this.tokenType = tokenType
    this.accessToken = accessToken
  }
}
