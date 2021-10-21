import { names } from "$lib/domain/configuration/names"

type NamesDto = { body }

export const get = (): NamesDto => {
  return {
    body: names,
  }
}
