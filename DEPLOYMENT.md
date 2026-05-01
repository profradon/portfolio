# Deployment Guide: Vercel (Frontend) + Render (Backend)

## Part 1: Frontend Deployment to Vercel

### Step 1: Push code to GitHub

```bash
cd /home/rustyradon/PORT1
git add .
git commit -m "Portfolio ready for deployment"
git push
```

### Step 2: Create Vercel account and connect GitHub

1. Go to [vercel.com](https://vercel.com)
2. Click **Sign Up** → choose **GitHub**
3. Authorize Vercel to access your GitHub repos
4. After login, click **Add New Project**

### Step 3: Import your repository

1. Find and select your PORT1 repository
2. Click **Import**
3. Configure project settings:
   - **Project Name**: `portfolio` (or your choice)
   - **Framework**: Select **Vite**
   - **Root Directory**: Leave as default (Vercel will auto-detect)

### Step 4: Configure environment variables for frontend

Click **Environment Variables** and add:

```
VITE_API_URL=https://your-backend.onrender.com
```

*(You'll get the Render backend URL in Part 2)*

### Step 5: Deploy

1. Click **Deploy**
2. Wait 2-5 minutes for build to complete
3. You'll get a live URL like: `https://portfolio-xyz.vercel.app`

**Your frontend is now live!**

---

## Part 2: Backend Deployment to Render

### Step 1: Prepare your repository

Ensure your `port-back` directory has:

- ✅ `Cargo.toml`
- ✅ `Cargo.lock` (commit to repo)
- ✅ `src/` folder with Rust code
- ✅ `.env.example` (for reference)

Make sure `.env` is in `.gitignore` and NOT committed.

```bash
# From root
git add port-back/
git commit -m "Add Rust backend for Render deployment"
git push
```

### Step 2: Create Render account and connect GitHub

1. Go to [render.com](https://render.com)
2. Click **Sign Up** → choose **GitHub**
3. Authorize Render to access your repos
4. Click **New +** → **Web Service**

### Step 3: Configure Render deployment

1. **Connect Repository**:
   - Select your PORT1 repository
   - Click **Connect**

2. **Service Settings**:
   - **Name**: `portfolio-backend`
   - **Region**: Choose closest to you (e.g., `us-east-1`)
   - **Branch**: `main`
   - **Runtime**: Select **Rust**
   - **Build Command**: 
     ```
     cd port-back && cargo build --release
     ```
   - **Start Command**: 
     ```
     cd port-back && cargo run --release
     ```

3. Click **Advanced** and configure:

### Step 4: Add environment variables

In **Environment**, add:

```
MONGODB_URI=<your-mongodb-connection-string>
DATABASE_NAME=portfolio
HOST=0.0.0.0
PORT=3000
```

### Step 5: How to get MongoDB connection string

**Option A: MongoDB Atlas (Cloud) - Recommended**

1. Go to [mongodb.com/cloud](https://mongodb.com/cloud)
2. Sign up for free tier
3. Create a cluster (choose your region)
4. In **Connect** → select **Drivers**
5. Copy connection string:
   ```
   mongodb+srv://<username>:<password>@cluster0.mongodb.net/portfolio?retryWrites=true&w=majority
   ```
6. Replace `<username>` and `<password>` with actual credentials
7. Paste into Render environment as `MONGODB_URI`

**Option B: Self-hosted MongoDB**

If running MongoDB locally:
```
MONGODB_URI=mongodb://localhost:27017
```

*(Not recommended for production; use Atlas)*

### Step 6: Deploy

1. After adding environment variables, click **Create Web Service**
2. Render will start building (5-10 minutes for Rust)
3. Watch the **Logs** tab for build progress
4. Once successful, you'll see:
   ```
   Server running on http://0.0.0.0:3000
   ```

5. Your backend URL will be:
   ```
   https://portfolio-backend.onrender.com
   ```

---

## Part 3: Connect Frontend to Backend

### Step 1: Update Vercel environment variable

1. Go to your [Vercel Dashboard](https://vercel.com/dashboard)
2. Select your `portfolio` project
3. Go to **Settings** → **Environment Variables**
4. Update `VITE_API_URL` with your Render backend:
   ```
   VITE_API_URL=https://portfolio-backend.onrender.com
   ```
5. Click **Save**

### Step 2: Trigger new deployment

1. Go to **Deployments**
2. Click the three dots on latest deployment
3. Click **Redeploy**

Now your frontend will call the live backend!

---

## Part 4: Test everything

### Test backend is working

```bash
curl https://portfolio-backend.onrender.com/api/blogs
```

You should get JSON response (empty array if no data yet).

### Test frontend is calling backend

1. Visit your Vercel URL: `https://portfolio-xyz.vercel.app`
2. Open **Developer Console** (F12)
3. Check **Network** tab for API calls
4. Should see requests to your Render backend

---

## Common Issues & Fixes

### Backend failing to start on Render

**Problem**: Build succeeds but service crashes

**Fix**:
1. Check **Logs** tab for error messages
2. Ensure `MONGODB_URI` is set correctly (test in MongoDB Compass)
3. Verify `HOST=0.0.0.0` is set (not `127.0.0.1`)

### Vercel shows API errors

**Problem**: Frontend can't reach backend

**Fix**:
1. Check `VITE_API_URL` is set in Vercel environment
2. Redeploy Vercel project
3. Check browser console for CORS errors
4. If CORS error, backend has issue (check Render logs)

### MongoDB connection timeout

**Problem**: Backend can't connect to MongoDB

**Fix**:
- Verify `MONGODB_URI` format: `mongodb+srv://user:pass@cluster.mongodb.net/dbname`
- Check credentials are URL-encoded (special chars like `@` need encoding)
- Whitelist Render IP in MongoDB Atlas:
  1. Go to **Network Access** → **Add IP Address**
  2. Click **Allow Access from Anywhere** (temporary, or add Render IPs)

### Render service keeps restarting

**Problem**: Free tier restarts after 15 mins of inactivity

**Solution**: 
- Use **Render Cron Job** to ping your backend every 10 mins, or
- Upgrade to **Paid Plan** ($7/month) for always-on service

---

## Final Checklist

- [ ] Frontend deployed to Vercel (live URL working)
- [ ] Backend deployed to Render (API responding)
- [ ] MongoDB Atlas cluster created and connected
- [ ] `VITE_API_URL` set in Vercel to Render backend
- [ ] Frontend making API calls to backend
- [ ] Admin login working
- [ ] Can create/edit/delete content from admin panel

---

## Monitoring & Maintenance

**Vercel Dashboard**: https://vercel.com/dashboard
- View logs: **Deployments** tab
- Auto-redeploys on push to main branch

**Render Dashboard**: https://dashboard.render.com
- View logs: **Logs** tab
- Monitor uptime: **Health** tab
- Manual redeploy: **Manual Deploy** button

**MongoDB Atlas**: https://cloud.mongodb.com
- Check connection: **Connect** → **Compass**
- Monitor usage: **Billing** tab

