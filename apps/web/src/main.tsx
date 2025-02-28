import ReactDOM from "react-dom/client"
import App from "@/app/router"

import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { AuthProvider } from "@/features/auth/context"
import { BrowserRouter } from "react-router-dom"

const rootElement = document.getElementById("root")!
const root = ReactDOM.createRoot(rootElement)

import "./main.css"

const queryClient = new QueryClient()

root.render(
	<BrowserRouter>
		<QueryClientProvider client={queryClient}>
			<AuthProvider>
				<App />
			</AuthProvider>
		</QueryClientProvider>
	</BrowserRouter>,
)
