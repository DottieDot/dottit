import { useQuery } from '@apollo/client'
import ThreadCard from '../ThreadCard'
import { query, RequestVariables, ResponseData } from './api'
import AutoSizer from 'react-virtualized-auto-sizer'
import { memo, useCallback, useMemo, useRef } from 'react'
import DynamicSizeList from '../DynamicSizeList'
import { Box, LinearProgress } from '@mui/material'
import { ListOnScrollProps, VariableSizeList } from 'react-window'

interface PostFeedProps {
  board: string,
  onThreadSelected: (threadId: string) => void
}

function ThreadFeed({ board, onThreadSelected }: PostFeedProps) {
  const ref = useRef<VariableSizeList>(null)
  // eslint-disable-next-line
  const date = useMemo(() => new Date(), [ board ])
  const { data, loading, fetchMore } = useQuery<ResponseData, RequestVariables>(query, {
    variables: {
      board,
      count: 20,
      first: date
    }
  })

  const next = data?.getBoardByName?.threads.next

  const loadMore = useCallback(() => {
    const fn = async () => {
      if (!loading && next !== null) {
        await fetchMore({ variables: { first: next }})
      }
    }
    fn().then()
  }, [ fetchMore, next, loading ])

  const handleScroll = useCallback((e: ListOnScrollProps) => {
    if (!ref.current) {
      return
    }

    // Super dirty hack here
    // @ts-ignore
    const scrollable: HTMLDivElement = ref.current?._outerRef.childNodes[0]

    if (e.scrollOffset + +ref.current.props.height === scrollable.clientHeight) {
      loadMore()
    }
  }, [ loadMore,ref ])


  const threads = useMemo(() => {
    return data?.getBoardByName.threads.items ?? []
  }, [ data ])

  return (
    <Box
      sx={{
        position: 'relative',
        height:   '100%'
      }}
    >
      {loading && (
        <LinearProgress
          sx={{
            position: 'absolute',
            top:      0,
            width:    '100%'
          }}
        />
      )}

      <AutoSizer>
        {({ width, height }) => (
          <DynamicSizeList
            height={height}
            items={threads}
            onScroll={handleScroll}
            ref={ref}
            renderItem={(index) => (
              <ThreadCard onClick={onThreadSelected} thread={threads[index]} />
            )}
            width={width}
          />
        )}
      </AutoSizer>
    </Box>
  )
}

export default memo(ThreadFeed)

