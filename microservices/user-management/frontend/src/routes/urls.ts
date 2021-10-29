import { urls } from "$lib/domain/configuration/public/urls"

type UrlsDto = { body }

export const get = (): UrlsDto => {
  return {
    body: urls,
  }
}
