# Quick Reference: Deployment URLs & Accounts

## Services to Set Up

| Service | Type | Sign Up Link | Free Tier |
|---------|------|-------------|-----------|
| **Vercel** | Frontend Hosting | https://vercel.com | Yes (unlimited) |
| **Render** | Backend Hosting | https://render.com | Yes (auto-sleep) |
| **MongoDB Atlas** | Database | https://mongodb.com/cloud | Yes (512MB) |
| **GitHub** | Code Repo | https://github.com | Yes (public repos) |

---

## Deployment Sequence

```
1. Push code to GitHub
   └─ git push

2. Set up MongoDB Atlas
   └─ Create cluster → Get connection string

3. Deploy Backend to Render
   └─ Connect GitHub repo
   └─ **Important**: Set Root Directory to `port-back`
   └─ Set MONGODB_URI env var
   └─ Deploy → Get backend URL

4. Deploy Frontend to Vercel
   └─ Connect GitHub repo
   └─ Set VITE_API_URL env var (point to Render)
   └─ Deploy

5. Test
   └─ Visit frontend URL
   └─ Check API calls in browser console
```

---

## Dashboards After Deployment

Once deployed, track everything here:

1. **Vercel Dashboard**: https://vercel.com/dashboard
   - View live URL of frontend
   - Check build logs
   - Manage environment variables
   - View deployment history

2. **Render Dashboard**: https://dashboard.render.com
   - View live URL of backend
   - Check service logs
   - Monitor uptime
   - Manage environment variables

3. **MongoDB Atlas**: https://cloud.mongodb.com
   - View database
   - Monitor usage (512MB free limit)
   - Manage users
   - Check connection

---

## Local Testing Before Deployment

Before pushing to production:

```bash
# Test frontend build
npm run build

# Test backend build
cd port-back && cargo build --release

# Test linting
npm run lint

# Push to GitHub
git push
```

---

## Troubleshooting Quick Links

| Issue | Solution |
|-------|----------|
| Frontend can't reach backend | Check `VITE_API_URL` in Vercel env vars |
| Backend won't start | Check MongoDB connection string in Render logs |
| MongoDB connection timeout | Whitelist Render IP in MongoDB Atlas |
| CORS errors | Check Render logs for backend issues |
| Render keeps restarting | Free tier auto-sleeps; upgrade or use cron job |

---

## After Deployment

**Daily Checks:**
- Visit your live frontend URL
- Check admin panel login works
- Submit test blog post/project

**Weekly:**
- Check Render logs for errors
- Monitor MongoDB usage (< 512MB)
- Check Vercel build status

**When Adding Features:**
- Test locally first
- Commit to git `git push`
- Vercel auto-redeploys
- Render auto-redeploys on push

---

## Files You Created

- `DEPLOYMENT.md` - Full step-by-step guide
- `ENV_SETUP.md` - Environment variable reference
- `deploy-check.sh` - Pre-deployment checklist script
- `FRONTEND/.env.example` - Frontend env template
- `port-back/.env.example` - Backend env template

📖 **Read DEPLOYMENT.md for detailed instructions**

