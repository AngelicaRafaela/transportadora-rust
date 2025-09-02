-- Clientes
CREATE TABLE IF NOT EXISTS clientes (
  id UUID PRIMARY KEY,
  nome_razao TEXT NOT NULL,
  cpf TEXT,
  cnpj TEXT,
  email TEXT,
  telefone TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Motoristas
CREATE TABLE IF NOT EXISTS motoristas (
  id UUID PRIMARY KEY,
  nome TEXT NOT NULL,
  cnh TEXT NOT NULL,
  telefone TEXT,
  ativo BOOLEAN NOT NULL DEFAULT true,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Caminhões
CREATE TABLE IF NOT EXISTS caminhoes (
  id UUID PRIMARY KEY,
  placa TEXT NOT NULL UNIQUE,
  marca TEXT,
  modelo TEXT,
  ano INT,
  cor TEXT,
  placa_carreta TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Viagens
CREATE TABLE IF NOT EXISTS viagens (
  id UUID PRIMARY KEY,
  origem_cidade TEXT NOT NULL,
  destino_cidade TEXT NOT NULL,
  data_partida DATE,
  data_chegada DATE,
  status TEXT NOT NULL DEFAULT 'aberta',
  motorista_id UUID NOT NULL REFERENCES motoristas(id),
  caminhao_id UUID NOT NULL REFERENCES caminhoes(id),
  cliente_id UUID NOT NULL REFERENCES clientes(id),
  distancia_km DOUBLE PRECISION,
  valor_frete DOUBLE PRECISION,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_viagens_motorista ON viagens(motorista_id);
CREATE INDEX IF NOT EXISTS idx_viagens_cliente   ON viagens(cliente_id);
CREATE INDEX IF NOT EXISTS idx_viagens_caminhao  ON viagens(caminhao_id);
