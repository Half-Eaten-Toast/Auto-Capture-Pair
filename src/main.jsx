import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import { SnackbarProvider } from "notistack";

createRoot(document.getElementById("root")).render(
	<SnackbarProvider>
		<App />
	</SnackbarProvider>,
);
