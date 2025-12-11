import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";
import "./auth.css";

export function Auth() {
  const navigate = useNavigate();

  // estado principal
  const [mode, setMode] = useState<"login" | "register">("login");

  // campos do formulário
  const [user, setUser] = useState("");
  const [pass, setPass] = useState("");
  const [confirm, setConfirm] = useState("");

  // ui
  const [error, setError] = useState("");
  const [showPass, setShowPass] = useState(false);
  const [showConfirm, setShowConfirm] = useState(false);

  // modal de sucesso após registro
  const [successModal, setSuccessModal] = useState(false);

  // validações derivadas
  const passwordsMatch =
    mode === "register" && pass && confirm && pass === confirm;

  const passwordsMismatch =
    mode === "register" && pass && confirm && pass !== confirm;

  // limpa quando troca de modo
  function resetForm() {
    setUser("");
    setPass("");
    setConfirm("");
    setError("");
    setShowPass(false);
    setShowConfirm(false);
  }

  // SUBMIT
  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError("");

    if (user.trim().length < 3) {
      setError("O nome de usuário deve ter pelo menos 3 caracteres.");
      return;
    }

    if (pass.length < 6) {
      setError("A senha deve ter pelo menos 6 caracteres.");
      return;
    }

    if (mode === "register" && pass !== confirm) {
      setError("As senhas não coincidem.");
      return;
    }

    try {
      if (mode === "register") {
        await invoke("register_user", { username: user, password: pass });

        // abre modal de sucesso
        setSuccessModal(true);

        // limpa e troca para login
        setMode("login");
        resetForm();
        return;
      }

      // login
      const session = await invoke("login_user", {
        username: user,
        password: pass,
      });

      localStorage.setItem("session", JSON.stringify(session));
      navigate("/dashboard");

    } catch (err: any) {
      setError(String(err));
    }
  }

  return (
    <>
      {/* =========================== */}
      {/* MODAL DE SUCESSO */}
      {/* =========================== */}
      {successModal && (
        <div className="auth-modal-backdrop">
          <div className="auth-modal">
            <h2 className="auth-modal-title">Conta criada!</h2>
            <p className="auth-modal-text">
              Sua conta foi registrada com sucesso.  
              Agora você pode entrar normalmente.
            </p>

            <div className="auth-modal-footer">
              <button
                className="auth-modal-btn-primary"
                onClick={() => setSuccessModal(false)}
              >
                OK
              </button>
            </div>
          </div>
        </div>
      )}

      {/* =========================== */}
      {/* TELA PRINCIPAL */}
      {/* =========================== */}
      <div className="auth-wrapper">
        <div className="auth-window">

          <h1 className="auth-title">Password Manager</h1>

          {/* Tabs */}
          <div className="auth-tabs">
            <button
              className={`auth-tab ${mode === "login" ? "active" : ""}`}
              onClick={() => { setMode("login"); resetForm(); }}
            >
              Entrar
            </button>

            <button
              className={`auth-tab ${mode === "register" ? "active" : ""}`}
              onClick={() => { setMode("register"); resetForm(); }}
            >
              Criar Conta
            </button>
          </div>

          {error && <p className="auth-error">{error}</p>}

          <form className="auth-form" onSubmit={handleSubmit}>
            
            {/* Usuário */}
            <input
              type="text"
              placeholder="Usuário"
              value={user}
              onChange={(e) => setUser(e.target.value)}
            />

            {/* Senha */}
            <div
              className={`auth-input-wrap ${passwordsMatch ? "match" : passwordsMismatch ? "no-match" : ""}`}
            >
              <input
                type={showPass ? "text" : "password"}
                placeholder="Senha"
                value={pass}
                onChange={(e) => setPass(e.target.value)}
                className={passwordsMatch ? "match" : passwordsMismatch ? "no-match" : ""}
              />

              <button
                type="button"
                className="show-pass-btn"
                onClick={() => setShowPass((v) => !v)}
              >
                {showPass ? (
                  <svg width="20" height="20" viewBox="0 0 24 24">
                    <path d="M17.94 17.94A10 10 0 0 1 12 20c-5.5 0-10-4-11-8 1-4 5-8 11-8a10 10 0 0 1 6 2M1 1l22 22"
                      stroke="currentColor" strokeWidth="2" fill="none" />
                  </svg>
                ) : (
                  <svg width="20" height="20" viewBox="0 0 24 24">
                    <path d="M1 12s4-7 11-7 11 7 11 7-4 7-11 7-11-7-11-7Z"
                      stroke="currentColor" strokeWidth="2" fill="none" />
                    <circle cx="12" cy="12" r="3"
                      stroke="currentColor" strokeWidth="2" fill="none" />
                  </svg>
                )}
              </button>
            </div>

            {/* Confirmar senha */}
            {mode === "register" && (
              <div
                className={`auth-input-wrap ${passwordsMatch ? "match" : passwordsMismatch ? "no-match" : ""}`}
              >
                <input
                  type={showConfirm ? "text" : "password"}
                  placeholder="Confirmar senha"
                  value={confirm}
                  onChange={(e) => setConfirm(e.target.value)}
                  className={passwordsMatch ? "match" : passwordsMismatch ? "no-match" : ""}
                />

                <button
                  type="button"
                  className="show-pass-btn"
                  onClick={() => setShowConfirm((v) => !v)}
                >
                  {showConfirm ? (
                    <svg width="20" height="20" viewBox="0 0 24 24">
                      <path d="M17.94 17.94A10 10 0 0 1 12 20c-5.5 0-10-4-11-8 1-4 5-8 11-8a10 10 0 0 1 6 2M1 1l22 22"
                        stroke="currentColor" strokeWidth="2" fill="none" />
                    </svg>
                  ) : (
                    <svg width="20" height="20" viewBox="0 0 24 24">
                      <path d="M1 12s4-7 11-7 11 7 11 7-4 7-11 7-11-7-11-7Z"
                        stroke="currentColor" strokeWidth="2" fill="none" />
                      <circle cx="12" cy="12" r="3"
                        stroke="currentColor" strokeWidth="2" fill="none" />
                    </svg>
                  )}
                </button>
              </div>
            )}

            <button className="auth-btn" type="submit">
              {mode === "login" ? "Entrar" : "Criar Conta"}
            </button>
          </form>
        </div>
      </div>
    </>
  );
}
