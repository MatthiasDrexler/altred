import { Writable, writable } from "svelte/store"
import type { AccessToken } from "src/lib/domain/entities/accessToken"

class AuthenticationService {
  public token: Writable<AccessToken | undefined> = writable(undefined)

  constructor() {
    this.token = undefined
  }

  public isAuthenticated(): boolean {
    return this.token != undefined
  }
}

export const authenticationService: AuthenticationService = new AuthenticationService()
