import { Fragment, memo, useCallback } from 'react'
import { ResponseComment } from './api'
import Comment from '../Comment'
import { Stack, SxProps } from '@mui/material'
import { VisibilityTrigger } from '../index'

interface Props {
  comments: ResponseComment[]
  sx?: SxProps
  loadMore: () => void
}

function Comments({ comments, sx, loadMore }: Props) {
  const handleVisibilityChange = useCallback((visible: boolean) => {
    if (visible) {
      loadMore()
    }
  }, [ loadMore ])

  return (
    <Fragment>
      <Stack gap={2} sx={sx}>
        {comments.map(comment => (
          <Comment key={comment.id} {...comment} />
        ))}
      </Stack>

      <VisibilityTrigger onVisibilityChange={handleVisibilityChange} />
    </Fragment>
  )
}

export default memo(Comments)
