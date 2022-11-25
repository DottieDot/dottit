import { RouterProvider } from 'react-router'
import { BoardPage, LoginSignupPage } from './pages'
import { createBrowserRouter } from 'react-router-dom'

function Router() {
  const router = createBrowserRouter([
    {
      path:     '/b/:board',
      element:  <BoardPage />,
      children: [
        {
          path:    't/:thread',
          element: <BoardPage />
        }
      ]
    },
    {
      path:    '/login',
      element: <LoginSignupPage />
    }
  ])

  return (
    <RouterProvider router={router} />
  )
}

export default Router
