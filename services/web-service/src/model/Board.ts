import { Page } from './Page'

export interface Board {
  id: string
  name: string
}

export interface BoardThreads<T> {
  threads: Page<T, Date>
}

export interface BoardModerators<U> {
  moderators: U[]
}
