import { Box, Fab, Grid } from '@mui/material'
import { BoardHeader, ThreadFeed } from '../components'
import { Add as NewThreadIcon } from '@mui/icons-material'
import { useParams } from 'react-router'
import { memo, useCallback, useState } from 'react'
import { NewThreadDialog } from '../dialogs'

function BoardPage() {
  const { board } = useParams<{ board: string }>()
  const [ newThread, setNewThread ] = useState(false)

  const handleNewThread = useCallback(() => {
    setNewThread(true)
  }, [])


  if (typeof board !== 'string') return null

  return (
    <Grid sx={{ height: '100%' }} container>
      <NewThreadDialog onClose={setNewThread} open={newThread} />

      <Grid
        md={4}
        sx={{
          position: 'relative',
          height:   '100%'
        }}
        xs={12}
        item
      >
        <Box
          sx={{
            overflowY: 'auto',
            height:    '100%'
          }}
        >
          <BoardHeader board={board} />
          <ThreadFeed board={board} />
        </Box>

        <Fab
          color="primary"
          onClick={handleNewThread}
          sx={{
            position: 'absolute',
            right:    0,
            bottom:   0,
            mb:       2,
            mr:       2
          }}
        >
          <NewThreadIcon />
        </Fab>
      </Grid>

      <Grid
        md={8}
        sx={{ borderLeft: '2px solid #444' }}
        xs={0}
        item
      >

      </Grid>
    </Grid>
  )
}

export default memo(BoardPage)
