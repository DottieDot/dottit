import { Navigate, RouterProvider } from 'react-router'
import { BoardPage, LoginSignupPage, UserPage } from './pages'
import { createBrowserRouter } from 'react-router-dom'
import LoginForm from './pages/LoginSignupPage/LoginForm'
import SignupForm from './pages/LoginSignupPage/SignupForm'
import { StandardLayout } from './layouts'

function Router() {
  const router = createBrowserRouter([
    {
      path:     '/',
      element:  <StandardLayout />,
      children: [
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
          path:    '/u/:user',
          element: <UserPage />
        }
      ]
    },
    {
      path:     '/auth',
      element:  <LoginSignupPage />,
      children: [
        {
          element: <Navigate to="/auth/login" replace />,
          index:   true
        },
        {
          path:    'login',
          element: <LoginForm />
        },
        {
          path:    'signup',
          element: <SignupForm />
        }
      ]
    }
  ])

  return (
    <RouterProvider router={router} />
  )
}

export default Router
