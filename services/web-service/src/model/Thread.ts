
export interface Thread {
  id: string,
  boardId: string,
  title: string,
  text: string
}

export interface ThreadUser<T> {
  user: T
}
