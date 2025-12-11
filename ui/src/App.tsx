import { BrowserRouter, Routes, Route, Navigate, Outlet } from "react-router-dom";
import { Auth } from "./pages/Auth";
import { Dashboard } from "./pages/Dashboard";

function ProtectedRoute() {
  const raw = localStorage.getItem("session");

  if (!raw) {
    return <Navigate to="/" replace />;
  }

  let session;

  try {
    session = JSON.parse(raw);
  } catch {
    localStorage.removeItem("session");
    return <Navigate to="/" replace />;
  }

  // repassa a sessão via context-like prop
  return <Outlet context={{ session }} />;
}

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* Rota pública */}
        <Route path="/" element={<Auth />} />

        {/* Rota protegida */}
        <Route element={<ProtectedRoute />}>
          <Route
            path="/dashboard"
            element={<Dashboard />}
          />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
