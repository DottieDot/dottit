
export interface Comment {
  id: string
  threadId: string
  text: string
}

export interface CommentUser<T> {
  user: T
}
