export class AccessToken {
  constructor(private _accessToken: string) {}

  public get accessToken(): string {
    return this._accessToken
  }

  public set accessToken(value: string) {
    this._accessToken = value
  }
}
