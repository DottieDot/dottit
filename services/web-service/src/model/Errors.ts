
export interface ValidationError {
  message: string
  errors: FieldError[]
}

export interface FieldError {
  field: string
  errors: string[]
}

export interface AlreadyLoggedIn {
  message: string
}

export interface Unauthorized {
  message: string
}

export interface Unauthenticated {
  message: string
}

export interface LoginFailed {
  message: string
}
