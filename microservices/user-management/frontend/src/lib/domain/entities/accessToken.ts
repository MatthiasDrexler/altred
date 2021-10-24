export class AccessToken {
  constructor(private _tokenType: string, private _accessToken: string, private _expiresIn: number) {}

  public get tokenType(): string {
    return this._tokenType
  }

  public get accessToken(): string {
    return this._accessToken
  }

  public get expiresIn(): number {
    return this._expiresIn
  }
}
