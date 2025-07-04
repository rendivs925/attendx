-- Enums
CREATE TYPE role AS ENUM ('superadmin', 'developer', 'orgowner');
CREATE TYPE user_status AS ENUM ('active', 'inactive', 'suspended');
CREATE TYPE subscription_plan AS ENUM ('free', 'pro', 'premium', 'enterprise');
CREATE TYPE subscription_status AS ENUM ('active', 'expired', 'canceled', 'graceperiod');
CREATE TYPE attendance_method AS ENUM ('qrcode', 'facialrecognition', 'nfc', 'manual', 'gps', 'biometric');
CREATE TYPE attendance_status AS ENUM ('present', 'absent', 'late', 'excused');
CREATE TYPE attendance_type AS ENUM ('singlemark', 'doublemark');

-- Users
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  organization_id UUID NOT NULL,
  role role NOT NULL DEFAULT 'orgowner',
  status user_status NOT NULL DEFAULT 'active',
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Subscriptions
CREATE TABLE subscriptions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
  plan subscription_plan NOT NULL DEFAULT 'free',
  status subscription_status NOT NULL DEFAULT 'active',
  start_date TIMESTAMPTZ NOT NULL DEFAULT now(),
  expiry_date TIMESTAMPTZ NOT NULL
);

-- Organizations
CREATE TABLE organizations (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  owner_id UUID NOT NULL,
  logo_url TEXT NOT NULL DEFAULT '',
  max_users INTEGER NOT NULL DEFAULT 0,
  max_attendance_logs INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Attendances
CREATE TABLE attendances (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  clock_in TIMESTAMPTZ,
  clock_out TIMESTAMPTZ,
  date TIMESTAMPTZ NOT NULL,
  method attendance_method NOT NULL DEFAULT 'manual',
  status attendance_status NOT NULL DEFAULT 'absent',
  attendance_type attendance_type NOT NULL DEFAULT 'singlemark',
  lat DOUBLE PRECISION,
  long DOUBLE PRECISION,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
