import { Box, Fab, Grid } from '@mui/material'
import { useRouter } from 'next/router'
import { BoardHeader, ThreadFeed } from '../../components'
import { Add as NewThreadIcon } from '@mui/icons-material'
import { NewThreadDialog, NewThreadDialogInfo } from '../../dialogs'
import { useDialogs } from '../../hooks'
import { useCallback } from 'react'

export default function Board() {
  const router = useRouter()
  const { board } = router.query
  const { showDialog } = useDialogs()

  const handleNewThread = useCallback(() => {
    showDialog(NewThreadDialogInfo, undefined)
  }, [ showDialog ])

  if (typeof board !== 'string') return

  return (
    <Grid sx={{ height: '100%' }} container>
      {/*<NewThreadDialog />*/}

      <Grid
        md={4}
        sx={
          {
            position: 'relative',
            height:   '100%'
          }
        }
        xs={12}
        item
      >
        <Box
          sx={
            {
              overflowY: 'auto',
              height:    '100%'
            }
          }
        >
          <BoardHeader board={board} />

          <ThreadFeed board={board} />
        </Box>

        <Fab
          color="primary"
          onClick={handleNewThread}
          sx={
            {
              position: 'absolute',
              right:    0,
              bottom:   0,
              mb:       2,
              mr:       2
            }
          }
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
