import { useEffect, useState, useMemo, useCallback } from "react";
import { useOutletContext, useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";
import "./dashboard.css";

export interface Credential {
  id: string;
  name: string;
  username?: string;
  url?: string;
  updated_at: string;
}

interface SessionDTO {
  username: string;
  vault_id: string;
  vault_key: string;
  passphrase: string;
}

type DetailsMode = "view" | "edit" | "new" | "loading";

/* ============================================================
   TEMPO RELATIVO
============================================================ */
function timeAgo(date: string) {
  const diff = Date.now() - new Date(date).getTime();
  const sec = Math.floor(diff / 1000);
  const min = Math.floor(sec / 60);
  const hr = Math.floor(min / 60);
  const day = Math.floor(hr / 24);

  if (sec < 10) return "agora";
  if (sec < 60) return `há ${sec}s`;
  if (min < 60) return `há ${min} min`;
  if (hr < 24) return `há ${hr} h`;
  return `há ${day} dias`;
}

/* ============================================================
   DASHBOARD
============================================================ */
export function Dashboard() {
  const { session } = useOutletContext<{ session: SessionDTO }>();
  const navigate = useNavigate();

  function handleLogout() {
    localStorage.removeItem("session");
    navigate("/");
  }

  const [search, setSearch] = useState("");
  const [loading, setLoading] = useState(true);

  const [credentials, setCredentials] = useState<Credential[]>([]);
  const [selected, setSelected] = useState<Credential | null>(null);
  const [mode, setMode] = useState<DetailsMode>("view");

  /* SORT + FILTER PREMIUM */
  const [sort, setSort] = useState<"az" | "recent">("recent");
  const [filter, setFilter] = useState<"all" | "hasUser" | "hasUrl" | "noUser">(
    "all"
  );

  /* Load credentials */
  const loadCredentials = useCallback(async () => {
    try {
      setLoading(true);
      const list = await invoke<Credential[]>("list_credentials", { session });
      setCredentials(list);
    } finally {
      setLoading(false);
    }
  }, [session]);

  useEffect(() => {
    loadCredentials();
  }, [loadCredentials]);

  /* FILTER + SORT + SEARCH */
  const filtered = useMemo(() => {
    let list = [...credentials];

    if (search.trim()) {
      list = list.filter((c) =>
        c.name.toLowerCase().includes(search.toLowerCase())
      );
    }

    switch (filter) {
      case "hasUser":
        list = list.filter((c) => c.username);
        break;
      case "hasUrl":
        list = list.filter((c) => c.url);
        break;
      case "noUser":
        list = list.filter((c) => !c.username);
        break;
    }

    if (sort === "az") {
      list.sort((a, b) => a.name.localeCompare(b.name));
    } else if (sort === "recent") {
      list.sort(
        (a, b) =>
          new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime()
      );
    }

    return list;
  }, [credentials, search, filter, sort]);

  /* Select */
  function handleSelect(cred: Credential) {
    setSelected(null);
    setMode("loading");
    setTimeout(() => {
      setSelected(cred);
      setMode("view");
    }, 220);
  }

  function handleNew() {
    setSelected(null);
    setMode("new");
  }

  return (
    <div className="dash-shell">
      <header className="dash-topbar">
        <div className="dash-topbar-left">
          <span className="topbar-logo-dot" />
          <span className="topbar-title">Password Manager</span>
        </div>

        <div className="dash-topbar-right">
          <div className="topbar-user">
            <div className="topbar-avatar">
              {session.username.charAt(0).toUpperCase()}
            </div>
            <span className="topbar-username">{session.username}</span>
          </div>

          {/* LOGOUT */}
          <button
            className="topbar-logout-btn"
            onClick={handleLogout}
            title="Sair"
          >
            <svg viewBox="0 0 24 24" className="icon-svg">
              <path
                d="M16 17l5-5-5-5"
                strokeWidth="1.7"
                stroke="currentColor"
                fill="none"
                strokeLinecap="round"
              />
              <path
                d="M21 12H9"
                strokeWidth="1.7"
                stroke="currentColor"
                strokeLinecap="round"
              />
              <path
                d="M4 4h7v16H4z"
                strokeWidth="1.7"
                stroke="currentColor"
                fill="none"
              />
            </svg>
          </button>
        </div>
      </header>

      <div className="dash-wrapper">
        {/* =========================================
            COLUNA 1
        ========================================= */}
        <aside className="dash-col dash-col-left">
          <div className="dash-left-inner">
            <div>
              <h2 className="dash-panel-title">Cofre</h2>
              <p className="dash-panel-subtitle">Gerencie suas credenciais.</p>
            </div>

            <button className="dash-btn-primary" onClick={handleNew}>
              <span className="btn-plus">+</span>
              Nova credencial
            </button>

            <nav className="dash-nav">
              <button className="dash-nav-item active">
                <span className="nav-dot" />
                Todas as credenciais
              </button>
            </nav>
          </div>
        </aside>

        {/* =========================================
            COLUNA 2
        ========================================= */}
        <aside className="dash-col dash-col-middle">
          <div className="dash-middle-card">
            {/* SEARCH */}
            <div className="dash-search-wrap">
              <div className="dash-search-icon">
                <svg viewBox="0 0 24 24" className="icon-svg">
                  <circle
                    cx="11"
                    cy="11"
                    r="6"
                    stroke="currentColor"
                    strokeWidth="1.6"
                    fill="none"
                  />
                  <line
                    x1="16"
                    y1="16"
                    x2="21"
                    y2="21"
                    stroke="currentColor"
                    strokeWidth="1.6"
                  />
                </svg>
              </div>
              <input
                type="text"
                className="dash-search"
                placeholder="Buscar credenciais…"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
              />
            </div>

            {/* SORT + FILTER PREMIUM */}
            <div className="dash-filters-premium">
              {/* SORT */}
              <div className="filter-row">
                <span className="filter-label">Ordenar:</span>

                <button
                  className={`filter-pill ${sort === "az" ? "active" : ""}`}
                  onClick={() => setSort("az")}
                >
                  <svg viewBox="0 0 24 24" className="icon-svg">
                    <path
                      d="M7 7h10M7 12h10M7 17h10"
                      strokeWidth="1.6"
                      stroke="currentColor"
                      strokeLinecap="round"
                    />
                  </svg>
                  A–Z
                </button>

                <button
                  className={`filter-pill ${sort === "recent" ? "active" : ""}`}
                  onClick={() => setSort("recent")}
                >
                  <svg viewBox="0 0 24 24" className="icon-svg">
                    <circle
                      cx="12"
                      cy="12"
                      r="9"
                      stroke="currentColor"
                      strokeWidth="1.6"
                      fill="none"
                    />
                    <path
                      d="M12 6v6l4 2"
                      stroke="currentColor"
                      strokeWidth="1.6"
                      strokeLinecap="round"
                    />
                  </svg>
                  Recentes
                </button>
              </div>

              {/* FILTER */}
              <div className="filter-row">
                <span className="filter-label">Filtrar:</span>

                <button
                  className={`filter-pill ${filter === "all" ? "active" : ""
                    }`}
                  onClick={() => setFilter("all")}
                >
                  Todos
                </button>

                <button
                  className={`filter-pill ${filter === "hasUser" ? "active" : ""
                    }`}
                  onClick={() => setFilter("hasUser")}
                >
                  Logins
                </button>

                <button
                  className={`filter-pill ${filter === "hasUrl" ? "active" : ""
                    }`}
                  onClick={() => setFilter("hasUrl")}
                >
                  Sites
                </button>

                <button
                  className={`filter-pill ${filter === "noUser" ? "active" : ""
                    }`}
                  onClick={() => setFilter("noUser")}
                >
                  Sem usuário
                </button>
              </div>
            </div>

            {/* LISTA */}
            <div className="dash-list">
              {loading && (
                <div className="dash-empty">
                  <div className="loader-spinner"></div>
                  <p>Carregando…</p>
                </div>
              )}

              {!loading && filtered.length === 0 && (
                <div className="dash-empty">
                  <p>Nenhuma credencial encontrada.</p>
                </div>
              )}

              {!loading &&
                filtered.map((cred) => (
                  <button
                    key={cred.id}
                    className={`dash-list-item ${selected?.id === cred.id && mode !== "loading"
                        ? "active"
                        : ""
                      }`}
                    onClick={() => handleSelect(cred)}
                  >
                    <CredentialAvatar name={cred.name} url={cred.url} />

                    <div className="item-content">
                      <span className="item-title">{cred.name}</span>
                      {cred.username && (
                        <span className="item-sub">{cred.username}</span>
                      )}
                    </div>

                    <div className="item-meta">
                      <span className="item-meta-value">
                        {timeAgo(cred.updated_at)}
                      </span>
                    </div>
                  </button>
                ))}
            </div>
          </div>
        </aside>

        {/* =========================================
            COLUNA 3 — DETAILS
        ========================================= */}
        <section className="dash-col dash-col-right">
          {mode === "loading" && (
            <div className="dash-placeholder-card">
              <div className="dash-loading">
                <div className="loader-spinner"></div>
                <p>Carregando…</p>
              </div>
            </div>
          )}

          {mode === "new" && (
            <Details
              session={session}
              credential={null}
              mode="new"
              onRefresh={loadCredentials}
              onClose={() => {
                setMode("view");
                setSelected(null);
              }}
              onEditMode={() => setMode("edit")}
            />
          )}

          {!selected && mode !== "loading" && mode !== "new" && (
            <div className="dash-placeholder-card">
              <div className="dash-empty">
                <p>Selecione ou crie uma credencial.</p>
              </div>
            </div>
          )}

          {selected && mode !== "loading" && mode !== "new" && (
            <Details
              session={session}
              credential={selected}
              mode={mode}
              onRefresh={loadCredentials}
              onClose={() => {
                setMode("view");
                setSelected(null);
              }}
              onEditMode={() => setMode("edit")}
            />
          )}
        </section>
      </div>
    </div>
  );
}

/* ============================================================
   AVATAR (favicon automático)
============================================================ */
function CredentialAvatar({ name, url }: { name: string; url?: string }) {
  const [faviconUrl, setFaviconUrl] = useState<string | null>(null);

  useEffect(() => {
    if (!url) return setFaviconUrl(null);

    try {
      const domain = new URL(url).origin;
      const localIcon = `${domain}/favicon.ico`;

      fetch(localIcon, { method: "HEAD" })
        .then((res) => {
          if (res.ok) setFaviconUrl(localIcon);
          else
            setFaviconUrl(
              `https://www.google.com/s2/favicons?sz=128&domain=${domain}`
            );
        })
        .catch(() =>
          setFaviconUrl(
            `https://www.google.com/s2/favicons?sz=128&domain=${domain}`
          )
        );
    } catch {
      setFaviconUrl(null);
    }
  }, [url]);

  return faviconUrl ? (
    <div className="item-avatar">
      <img src={faviconUrl} className="item-avatar-img" alt={name} />
    </div>
  ) : (
    <div className="item-avatar item-avatar-fallback">
      {name.charAt(0).toUpperCase()}
    </div>
  );
}

/* ============================================================
   MODAL CONFIRMAR EXCLUSÃO (estilo Auth)
============================================================ */
interface ConfirmDeleteModalProps {
  credentialName?: string;
  onCancel: () => void;
  onConfirm: () => void;
}

function ConfirmDeleteModal({
  credentialName,
  onCancel,
  onConfirm,
}: ConfirmDeleteModalProps) {
  return (
    <div className="dash-modal-backdrop">
      <div className="dash-modal">
        <h3 className="dash-modal-title">Excluir credencial?</h3>

        <p className="dash-modal-text">
          A credencial{" "}
          {credentialName ? <strong>{credentialName}</strong> : "selecionada"}{" "}
          será removida do seu cofre. Esta ação não pode ser desfeita.
        </p>

        <div className="dash-modal-footer">
          <button
            type="button"
            className="modal-btn"
            onClick={onCancel}
          >
            Cancelar
          </button>
          <button
            type="button"
            className="modal-btn modal-btn-danger"
            onClick={onConfirm}
          >
            Excluir
          </button>
        </div>
      </div>
    </div>
  );
}

/* ============================================================
   DETAILS COMPONENT
============================================================ */
interface DetailsProps {
  session: SessionDTO;
  credential: Credential | null;
  mode: DetailsMode;
  onRefresh: () => void;
  onClose: () => void;
  onEditMode: () => void;
}

function Details({
  session,
  credential,
  mode,
  onRefresh,
  onClose,
  onEditMode,
}: DetailsProps) {
  const isNew = mode === "new";
  const isEditing = mode === "edit";

  const [loading, setLoading] = useState(!isNew);

  const [name, setName] = useState(credential?.name ?? "");
  const [username, setUsername] = useState(credential?.username ?? "");
  const [url, setUrl] = useState(credential?.url ?? "");
  const [password, setPassword] = useState("");
  const [notes, setNotes] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [showDeleteModal, setShowDeleteModal] = useState(false);

  useEffect(() => {
    if (isNew || !credential) return;

    let cancelled = false;
    const credentialId = credential.id;

    async function load() {
      setLoading(true);
      try {
        const full = await invoke<{
          password: string | null;
          notes: string | null;
        }>(
          "get_credential_full",
          { session, id: credentialId }
        );

        if (!cancelled) {
          setPassword(full.password ?? "");
          setNotes(full.notes ?? "");
        }
      } finally {
        if (!cancelled) setLoading(false);
      }
    }

    load();

    return () => {
      cancelled = true;
    };
  }, [credential, session, isNew]);

  const readOnly = !isNew && !isEditing;

  async function handleSave() {
    const payload = {
      session,
      name,
      username: username || null,
      url: url || null,
      password: password || null,
      notes: notes || null,
    };

    if (isNew) {
      await invoke("create_credential", payload);
    } else if (credential) {
      await invoke("update_credential", { ...payload, id: credential.id });
    }

    await onRefresh();
    onClose();
  }

  function handleDeleteClick() {
    if (!credential) return;
    setShowDeleteModal(true);
  }

  async function handleConfirmDelete() {
    if (!credential) return;

    setShowDeleteModal(false);
    await invoke("delete_credential", { session, id: credential.id });
    await onRefresh();
    onClose();
  }

  async function copy(val: string) {
    if (val) await navigator.clipboard.writeText(val);
  }

  return (
    <>
      <div className="details-card">
        {loading ? (
          <div className="dash-loading">
            <div className="loader-spinner"></div>
            <p>Carregando…</p>
          </div>
        ) : (
          <>
            {/* HEADER */}
            <div className="details-header">
              <div>
                <h2 className="details-title">
                  {isNew ? "Nova credencial" : name}
                </h2>

                {!isNew && credential?.url && (
                  <a
                    href={credential.url}
                    className="details-url"
                    onClick={(e) => {
                      e.preventDefault();
                      window.open(credential.url!, "_blank");
                    }}
                  >
                    {credential.url}
                  </a>
                )}
              </div>

              <div className="details-actions">
                {!isNew && !isEditing && (
                  <>
                    <button
                      className="icon-button"
                      onClick={onEditMode}
                      title="Editar"
                    >
                      <svg viewBox="0 0 24 24" className="icon-svg">
                        <path
                          d="M4 20h4l10-10-4-4L4 16v4z"
                          stroke="currentColor"
                          strokeWidth="1.6"
                          fill="none"
                        />
                      </svg>
                    </button>

                    <button
                      className="icon-button icon-danger"
                      onClick={handleDeleteClick}
                      title="Excluir"
                    >
                      <svg viewBox="0 0 24 24" className="icon-svg">
                        <path
                          d="M4 7h16"
                          stroke="currentColor"
                          strokeWidth="1.6"
                        />
                        <rect
                          x="6"
                          y="7"
                          width="12"
                          height="12"
                          rx="2"
                          stroke="currentColor"
                          strokeWidth="1.6"
                          fill="none"
                        />
                      </svg>
                    </button>
                  </>
                )}

                <button
                  className="icon-button"
                  onClick={onClose}
                  title="Fechar"
                >
                  <svg viewBox="0 0 24 24" className="icon-svg">
                    <line
                      x1="6"
                      y1="6"
                      x2="18"
                      y2="18"
                      stroke="currentColor"
                      strokeWidth="1.6"
                    />
                    <line
                      x1="6"
                      y1="18"
                      x2="18"
                      y2="6"
                      stroke="currentColor"
                      strokeWidth="1.6"
                    />
                  </svg>
                </button>
              </div>
            </div>

            {/* CAMPOS */}
            <div className="details-fields">
              <div className="details-field-group">
                <label>Nome</label>
                <input
                  value={name}
                  readOnly={readOnly}
                  onChange={(e) => setName(e.target.value)}
                />
              </div>

              <div className="details-field-group">
                <label>Usuário</label>
                <div className="field-inline">
                  <input
                    value={username}
                    readOnly={readOnly}
                    onChange={(e) => setUsername(e.target.value)}
                  />
                  {username && (
                    <button
                      className="icon-button"
                      onClick={() => copy(username)}
                      title="Copiar usuário"
                    >
                      <svg viewBox="0 0 24 24" className="icon-svg">
                        <rect
                          x="9"
                          y="9"
                          width="11"
                          height="11"
                          rx="2"
                          fill="none"
                          strokeWidth="1.6"
                          stroke="currentColor"
                        />
                        <rect
                          x="4"
                          y="4"
                          width="11"
                          height="11"
                          rx="2"
                          fill="none"
                          strokeWidth="1.6"
                          stroke="currentColor"
                        />
                      </svg>
                    </button>
                  )}
                </div>
              </div>

              <div className="details-field-group">
                <label>URL</label>
                <input
                  value={url}
                  readOnly={readOnly}
                  onChange={(e) => setUrl(e.target.value)}
                />
              </div>

              <div className="details-field-group">
                <label>Senha</label>
                <div className="field-inline">
                  <input
                    type={showPassword ? "text" : "password"}
                    value={password}
                    readOnly={readOnly}
                    onChange={(e) => setPassword(e.target.value)}
                  />

                  <button
                    className="icon-button"
                    onClick={() => setShowPassword((v) => !v)}
                    title={showPassword ? "Ocultar senha" : "Mostrar senha"}
                  >
                    {showPassword ? (
                      <svg viewBox="0 0 24 24" className="icon-svg">
                        <path
                          d="M3 3l18 18"
                          stroke="currentColor"
                          strokeWidth="1.6"
                        />
                        <path
                          d="M5 5c1.7-1.3 4-2 7-2 7 0 11 7 11 7a17.3 17.3 0 0 1-3.1 4.3"
                          strokeWidth="1.6"
                          stroke="currentColor"
                          fill="none"
                        />
                      </svg>
                    ) : (
                      <svg viewBox="0 0 24 24" className="icon-svg">
                        <path
                          d="M1 12s4-7 11-7 11 7 11 7-4 7-11 7S1 12 1 12Z"
                          strokeWidth="1.6"
                          stroke="currentColor"
                          fill="none"
                        />
                        <circle
                          cx="12"
                          cy="12"
                          r="3"
                          strokeWidth="1.6"
                          stroke="currentColor"
                          fill="none"
                        />
                      </svg>
                    )}
                  </button>

                  {password && (
                    <button
                      className="icon-button"
                      onClick={() => copy(password)}
                      title="Copiar senha"
                    >
                      <svg viewBox="0 0 24 24" className="icon-svg">
                        <rect
                          x="9"
                          y="9"
                          width="11"
                          height="11"
                          rx="2"
                          fill="none"
                          strokeWidth="1.6"
                          stroke="currentColor"
                        />
                        <rect
                          x="4"
                          y="4"
                          width="11"
                          height="11"
                          rx="2"
                          fill="none"
                          strokeWidth="1.6"
                          stroke="currentColor"
                        />
                      </svg>
                    </button>
                  )}
                </div>
              </div>

              <div className="details-field-group">
                <label>Notas</label>
                <textarea
                  value={notes}
                  readOnly={readOnly}
                  onChange={(e) => setNotes(e.target.value)}
                />
              </div>
            </div>

            {/* FOOTER */}
            <div className="details-footer">
              {isNew ? (
                <>
                  <button className="btn-secondary" onClick={onClose}>
                    Cancelar
                  </button>
                  <button className="btn-primary" onClick={handleSave}>
                    Criar
                  </button>
                </>
              ) : isEditing ? (
                <>
                  <button className="btn-secondary" onClick={onClose}>
                    Cancelar
                  </button>
                  <button className="btn-primary" onClick={handleSave}>
                    Salvar
                  </button>
                </>
              ) : (
                <button className="btn-secondary" onClick={onClose}>
                  Fechar
                </button>
              )}
            </div>
          </>
        )}
      </div>

      {/* MODAL DE EXCLUSÃO */}
      {showDeleteModal && (
        <ConfirmDeleteModal
          credentialName={name || credential?.name}
          onCancel={() => setShowDeleteModal(false)}
          onConfirm={handleConfirmDelete}
        />
      )}
    </>
  );
}
