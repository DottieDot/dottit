import { Fragment, memo, useCallback, useEffect, useState } from 'react'
import { Box, Button, Container, Divider, LinearProgress, Pagination, Typography } from '@mui/material'
import { Add as NewBoardIcon } from '@mui/icons-material'
import { useLazyQuery } from '@apollo/client'
import { getBoardsQuery, GetBoardsResponse } from './api'
import { Board } from '../../components'
import NewBoardDialog from './NewBoardDialog'
import { useUser } from '../../hooks'

const PAGE_SIZE = 50

function BoardsPage() {
  const [ getBoards, { data: newData, previousData, loading }] = useLazyQuery<GetBoardsResponse>(getBoardsQuery, {
    variables: {
      count: PAGE_SIZE,
      first: 0
    }
  })
  const { state: user } = useUser()
  const [ newBoardDialog, setNewBoardDialog ] = useState(false)
  const [ page, setPage ] = useState(1)

  const data = newData ?? previousData

  useEffect(() => {
    getBoards({ variables: { first: PAGE_SIZE * (page - 1) }}).then()
  }, [ getBoards, page ])

  const handleNewBoardClick = useCallback(() => {
    setNewBoardDialog(true)
  }, [])

  const handlePagination = useCallback((_: unknown, value: number) => {
    setPage(value)
  }, [])

  const pages = data?.boards.total ? Math.ceil(data?.boards.total / PAGE_SIZE) : undefined

  return (
    <Fragment>
      <LinearProgress sx={{ width: loading ? undefined : 0 }} />
      <NewBoardDialog onClose={setNewBoardDialog} open={newBoardDialog} />

      <Container maxWidth="lg" sx={{ py: 2 }}>
        <Box
          sx={{
            mb:             1,
            display:        'flex',
            flexDirection:  'row',
            alignItems:     'center',
            justifyContent: 'space-between'
          }}
        >
          <Typography component="h1" variant="h4">
            Boards
          </Typography>

          {user.user && (
            <Button onClick={handleNewBoardClick} startIcon={<NewBoardIcon />} variant="outlined">
              Create Board
            </Button>
          )}
        </Box>

        <Divider sx={{ mb: 2 }} />

        <Box
          sx={{
            display:             'grid',
            gridTemplateColumns: 'repeat(auto-fill, minmax(300px, 1fr))',
            gap:                 2,
            mb:                  2
          }}
        >
          {data?.boards.items.map(({ id, ...board }) => (
            <Board key={id} {...board} />
          ))}
        </Box>

        <Box
          sx={{
            display:        'flex',
            justifyContent: 'center'
          }}
        >
          <Pagination count={pages} onChange={handlePagination} page={page} />
        </Box>
      </Container>
    </Fragment>
  )
}

export default memo(BoardsPage)
