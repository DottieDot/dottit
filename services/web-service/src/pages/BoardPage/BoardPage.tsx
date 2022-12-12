import { Box, Fab, Grid } from '@mui/material'
import { BoardHeader, ThreadFeed, ThreadView } from '../../components'
import { Add as NewThreadIcon } from '@mui/icons-material'
import { useParams } from 'react-router'
import { memo, useCallback, useState } from 'react'
import { NewThreadDialog } from '../../dialogs'
import { useQuery } from '@apollo/client'
import { getBoardByNameQuery, GetBoardByNameResponse } from './api'
import { useSearchParams } from 'react-router-dom'
import { useUser } from '../../hooks'

function BoardPage() {
  const { board: boardName } = useParams<{ board: string, thread: string }>()
  const [ newThread, setNewThread ] = useState(false)
  const [ params, setParams ] = useSearchParams()
  const selectedThread = params.get('thread')
  const { data, loading } = useQuery<GetBoardByNameResponse>(getBoardByNameQuery, { variables: { name: boardName }})
  const { state: user } = useUser()

  const handleNewThread = useCallback(() => {
    setNewThread(true)
  }, [])

  const handleThreadSelected = useCallback((threadId: string) => {
    setParams({ thread: threadId })
  }, [ setParams ])


  if (typeof boardName !== 'string') return null

  if (loading || !data) return (
    <span>
      No Data
    </span>
  )

  if (!data.getBoardByName) return (
    <span>
      No board
    </span>
  )

  const board = data.getBoardByName

  return (
    <Grid sx={{ height: '100%' }} container>
      <NewThreadDialog
        board={board.id}
        onClose={setNewThread}
        open={newThread}
      />

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
            overflowY:        'auto',
            height:           '100%',
            display:          'grid',
            gridTemplateRows: 'auto 1fr'
          }}
        >
          <div>
            <BoardHeader board={board.name} />
          </div>

          <div>
            <ThreadFeed
              board={board.name}
              onThreadSelected={handleThreadSelected}
            />
          </div>
        </Box>

        {user.user && (
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
        )}
      </Grid>

      <Grid
        md={8}
        sx={{
          borderLeft:    '2px solid #444',
          height:        '100%',
          overflowX:     'auto',
          display:       'flex',
          flexDirection: 'column'
        }}
        xs={0}
        item
      >
        <ThreadView threadId={selectedThread} />
      </Grid>
    </Grid>
  )
}

export default memo(BoardPage)
