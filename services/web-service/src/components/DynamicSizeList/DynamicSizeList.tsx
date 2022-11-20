import { ListChildComponentProps, ListOnScrollProps, VariableSizeList } from 'react-window'
import { ForwardedRef, forwardRef, memo, ReactNode, useEffect, useImperativeHandle, useRef } from 'react'

export interface DynamicSizeListProps {
  width: number,
  height: number,
  items: unknown[],
  renderItem: (index: number) => ReactNode,
  onScroll?: (props: ListOnScrollProps) => void
}

function DynamicSizeList({ width, height, items, renderItem, onScroll } : DynamicSizeListProps, ref: ForwardedRef<unknown>) {
  const listRef = useRef<VariableSizeList>(null)
  const rowHeights = useRef<{[index:number]: number}>({})

  useImperativeHandle(ref, () => listRef.current)

  function getRowHeight(index: number) {
    return rowHeights.current[index] ?? 200
  }

  function setRowHeight(index: number, size: number) {
    rowHeights.current = {
      ...rowHeights.current,
      [index]: size
    }
    listRef.current?.resetAfterIndex(index)
  }

  function renderRow({ index, style }: ListChildComponentProps) {
    // eslint-disable-next-line react-hooks/rules-of-hooks
    const rowRef = useRef<HTMLDivElement>(null)

    // eslint-disable-next-line react-hooks/rules-of-hooks
    useEffect(() => {
      if (rowRef.current) {
        setRowHeight(index, rowRef.current.clientHeight)
      }
    }, [ rowRef, index ])

    return (
      <div style={style}>
        <div ref={rowRef}>
          {renderItem(index)}
        </div>
      </div>
    )
  }

  return (
    <VariableSizeList
      height={height}
      itemCount={items.length}
      itemSize={getRowHeight}
      onScroll={onScroll}
      ref={listRef}
      width={width}
    >
      {renderRow}
    </VariableSizeList>
  )
}

export default memo(forwardRef((props: DynamicSizeListProps, ref: ForwardedRef<unknown>) => DynamicSizeList(props, ref)))
