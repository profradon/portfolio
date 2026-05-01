# Environment Variables Reference

## Frontend (Vercel)

Add in Vercel Dashboard → Project Settings → Environment Variables:

```
VITE_API_URL=https://portfolio-backend.onrender.com
```

(Replace with your actual Render backend URL)

---

## Backend (Render)

Add in Render Dashboard → Environment:

```
MONGODB_URI=mongodb+srv://username:password@cluster0.mongodb.net/portfolio?retryWrites=true&w=majority
DATABASE_NAME=portfolio
HOST=0.0.0.0
PORT=3000
```

---

## Local Development

### Frontend (.env.local)

```
VITE_API_URL=http://localhost:3000
```

### Backend (.env)

```
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=portfolio
HOST=0.0.0.0
PORT=3000
```

---

## MongoDB Atlas Setup

1. Sign up: https://mongodb.com/cloud
2. Create Organization & Project
3. Create a Cluster:
   - Provider: AWS
   - Region: Pick closest to your users
   - Cluster Tier: M0 (Free)
   - Click "Create"
4. Wait 2-3 minutes for cluster to initialize
5. Create Database User:
   - Go to Database Access
   - Click "Add New Database User"
   - Username: `admin` (or custom)
   - Password: Generate (copy this!)
   - Click "Add User"
6. Add IP Whitelist:
   - Go to Network Access
   - Click "Add IP Address"
   - Choose "Allow Access from Anywhere" (0.0.0.0/0)
   - Click "Confirm"
7. Get Connection String:
   - Click "Connect" on your cluster
   - Choose "Drivers"
   - Copy connection string
   - Replace `<password>` with your database user password
   - Replace `myFirstDatabase` with `portfolio`

Example final string:
```
mongodb+srv://admin:MySecurePassword123@cluster0.abc123def.mongodb.net/portfolio?retryWrites=true&w=majority
```

Use this as `MONGODB_URI` in Render environment variables.

---

## Important Notes

- **Never commit `.env` files** to git (already in .gitignore)
- **Passwords in connection strings must be URL-encoded** (e.g., `@` becomes `%40`)
- **Use strong passwords** for MongoDB user
- **Free tier limits**: MongoDB Atlas free tier has 512MB storage
- **Render free tier**: Services auto-sleep after 15 mins of inactivity (use Paid $7/mo for always-on)
- **Vercel free tier**: Unlimited deployments, great for static hosting

