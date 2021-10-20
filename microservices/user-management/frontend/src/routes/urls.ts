import { urls } from "$lib/domain/configuration/urls"

type UrlsDto = { body }

export const get = (): UrlsDto => {
  return {
    body: urls,
  }
}
