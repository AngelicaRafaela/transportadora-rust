-- Tabela principal de motoristas
CREATE TABLE IF NOT EXISTS motoristas (
id SERIAL PRIMARY KEY,
nome TEXT NOT NULL,
cnh TEXT NOT NULL UNIQUE,
telefone TEXT,
criado_em TIMESTAMPTZ NOT NULL DEFAULT NOW(),
atualizado_em TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- Índices úteis
CREATE INDEX IF NOT EXISTS idx_motoristas_nome ON motoristas (nome);