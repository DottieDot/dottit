import { memo } from 'react'
import SelectAThread from './SelectAThread'
import Thread from './Thread'

export interface ThreadViewProps {
  threadId?: string | null
}

function ThreadView({ threadId }: ThreadViewProps) {
  if (!threadId) {
    return (
      <SelectAThread />
    )
  }

  return (
    <Thread threadId={threadId} />
  )
}

export default memo(ThreadView)
