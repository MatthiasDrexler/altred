import { names } from "$lib/domain/configuration/public/names"

type NamesDto = { body }

export const get = (): NamesDto => {
  return {
    body: names,
  }
}
