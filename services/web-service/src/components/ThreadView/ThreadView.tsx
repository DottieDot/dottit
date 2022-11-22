import { memo } from 'react'
import SelectAThread from './SelectAThread'

export interface ThreadViewProps {
  threadId?: string
}

function ThreadView({ threadId }: ThreadViewProps) {
  if (!threadId) {
    return (
      <SelectAThread />
    )
  }

  return (
    <div>
      Hello!
    </div>
  )
}

export default memo(ThreadView)
