import { RouterProvider } from 'react-router'
import { BoardPage } from './pages'
import { createBrowserRouter } from 'react-router-dom'

function Router() {
  const router = createBrowserRouter([
    {
      path:     '/b/:board',
      element:  <BoardPage />,
      children: [
        {
          path:    'thread/:thread',
          element: <BoardPage />
        }
      ]
    }
  ])

  return (
    <RouterProvider router={router} />
  )
}

export default Router
