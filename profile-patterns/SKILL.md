---
name: profile-patterns
description: Build user profile features following established patterns
license: MIT
compatibility: Claude Code
metadata:
  author: indx.sh
  version: "1.0"
  project: indx-web
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
---

# Profile Patterns Skill

## When to use this skill
Use when building or modifying user profile features. Invoke this skill when:
- Creating profile pages or components
- Adding social links (website, twitter, github)
- Building contribution displays
- Implementing follow/unfollow functionality
- Creating profile edit forms

## Core Pattern

User profiles follow this structure:

```
/u/[username]/
  ├── page.tsx              # Server component - data fetching
  └── components/
      ├── profile-layout.tsx    # Page wrapper
      ├── profile-hero.tsx      # Avatar, name, bio, stats
      └── contributions-list.tsx # User's content
```

## Profile Hero Component

The hero displays user identity and key stats:

```tsx
interface ProfileHeroProps {
  userId: string;
  name: string | null;
  username: string | null;
  image: string | null;
  bio: string | null;
  role: string;
  stats: {
    rules: number;
    mcps: number;
    saved: number;
    views: number;
    totalViews: number;
    totalCopies: number;
    totalLikes: number;
    followers: number;
    following: number;
  };
  avgRating: number;
  reviewCount: number;
  rank: number | null;
  joinedAt: Date;
  isOwner: boolean;
  isFollowing: boolean;
  currentUserId: string | null;
}

<ProfileHero
  userId={user.id}
  name={user.name}
  username={user.username}
  image={user.image}
  bio={user.bio}
  role={user.role}
  stats={stats}
  avgRating={avgRating}
  reviewCount={reviewCount}
  rank={user.leaderboardRank}
  joinedAt={user.createdAt}
  isOwner={isOwner}
  isFollowing={isFollowing}
  currentUserId={session?.user?.id ?? null}
/>
```

## Social Links

### Database Schema

Social links are stored in the User model:

```prisma
model User {
  website     String?
  twitter     String?
  github      String?
  linkedin    String?
}
```

### Display Pattern

```tsx
import { Globe, Twitter, Github, Linkedin } from 'lucide-react';

function SocialLinks({ user }: { user: UserWithSocials }) {
  const links = [
    { icon: Globe, href: user.website, label: 'Website' },
    { icon: Twitter, href: user.twitter ? `https://twitter.com/${user.twitter}` : null, label: 'Twitter' },
    { icon: Github, href: user.github ? `https://github.com/${user.github}` : null, label: 'GitHub' },
    { icon: Linkedin, href: user.linkedin ? `https://linkedin.com/in/${user.linkedin}` : null, label: 'LinkedIn' },
  ].filter(l => l.href);

  if (links.length === 0) return null;

  return (
    <div className="flex items-center gap-4">
      {links.map(({ icon: Icon, href, label }) => (
        <a
          key={label}
          href={href!}
          target="_blank"
          rel="noopener noreferrer"
          className={cn(
            'transition-colors',
            mode.color.text.muted,
            mode.state.hover.text
          )}
          aria-label={label}
        >
          <Icon className="h-4 w-4" />
        </a>
      ))}
    </div>
  );
}
```

### Edit Form for Social Links

```tsx
<SettingsSectionCard title="Social Links" description="Connect your online presence">
  <div className="space-y-4">
    <div>
      <label className={cn(mode.typography.label.md, mode.font, mode.color.text.muted)}>
        Website
      </label>
      <Input
        value={formData.website}
        onChange={(e) => setFormData({ ...formData, website: e.target.value })}
        placeholder="https://yoursite.com"
      />
    </div>
    <div>
      <label className={cn(mode.typography.label.md, mode.font, mode.color.text.muted)}>
        Twitter Username
      </label>
      <Input
        value={formData.twitter}
        onChange={(e) => setFormData({ ...formData, twitter: e.target.value })}
        placeholder="username"
        prefix="@"
      />
    </div>
    <div>
      <label className={cn(mode.typography.label.md, mode.font, mode.color.text.muted)}>
        GitHub Username
      </label>
      <Input
        value={formData.github}
        onChange={(e) => setFormData({ ...formData, github: e.target.value })}
        placeholder="username"
      />
    </div>
  </div>
</SettingsSectionCard>
```

## Contributions List

Display user's submitted content:

```tsx
interface Contribution {
  id: string;
  type: 'rule' | 'mcp';
  name: string;
  slug: string;
  description: string | null;
  tool: string;
  views: number;
  copies: number;
  isPinned: boolean;
  createdAt: Date;
}

<ContributionsList
  items={contributions}
  isOwner={isOwner}
  userId={user.id}
/>
```

### Contribution Card

```tsx
function ContributionCard({ item, isOwner, onPin, onUnpin }: ContributionCardProps) {
  return (
    <Link
      href={`/${item.type === 'rule' ? 'rules' : 'mcp'}/${item.slug}`}
      className={cn(
        'block border p-4 transition-colors',
        mode.radius,
        mode.color.border.default,
        mode.state.hover.card
      )}
    >
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <div className="flex items-center gap-2">
            {item.isPinned && (
              <Pin className={cn('h-4 w-4', mode.color.text.warning)} />
            )}
            <h3 className={cn(mode.typography.headline.sm, mode.font, mode.color.text.primary)}>
              {item.name}
            </h3>
          </div>
          <p className={cn('mt-1 line-clamp-2', mode.typography.body.md, mode.font, mode.color.text.muted)}>
            {item.description}
          </p>
        </div>

        <div className={cn('flex items-center gap-4', mode.typography.body.md, mode.font, mode.color.text.muted)}>
          <span>{item.views.toLocaleString()} views</span>
          <span>{item.copies.toLocaleString()} copies</span>
        </div>
      </div>
    </Link>
  );
}
```

## Follow/Unfollow

### API Route

```tsx
// POST /api/user/follow
export async function POST(req: Request) {
  const session = await auth();
  if (!session?.user) return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });

  const { userId } = await req.json();

  // Check existing follow
  const existing = await prisma.follow.findUnique({
    where: {
      followerId_followingId: {
        followerId: session.user.id,
        followingId: userId,
      },
    },
  });

  if (existing) {
    // Unfollow
    await prisma.follow.delete({ where: { id: existing.id } });
    await prisma.user.update({
      where: { id: userId },
      data: { followerCount: { decrement: 1 } },
    });
    return NextResponse.json({ following: false });
  }

  // Follow
  await prisma.follow.create({
    data: {
      followerId: session.user.id,
      followingId: userId,
    },
  });
  await prisma.user.update({
    where: { id: userId },
    data: { followerCount: { increment: 1 } },
  });

  return NextResponse.json({ following: true });
}
```

### Client Component

```tsx
function FollowButton({ userId, isFollowing: initial }: FollowButtonProps) {
  const [isFollowing, setIsFollowing] = React.useState(initial);
  const [isPending, setIsPending] = React.useState(false);

  async function handleFollow() {
    setIsPending(true);
    try {
      const response = await fetch('/api/user/follow', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ userId }),
      });
      const data = await response.json();
      setIsFollowing(data.following);
    } catch (error) {
      console.error('Follow failed:', error);
    } finally {
      setIsPending(false);
    }
  }

  return (
    <Button
      variant={isFollowing ? 'outline' : 'default'}
      onClick={handleFollow}
      disabled={isPending}
    >
      {isFollowing ? '> FOLLOWING' : '> FOLLOW'}
    </Button>
  );
}
```

## Profile Stats Display

```tsx
function ProfileStats({ stats }: { stats: ProfileStats }) {
  return (
    <div className={cn(
      'flex flex-wrap gap-6 border-t pt-4',
      mode.color.border.default
    )}>
      <StatItem label="Rules" value={stats.rules} />
      <StatItem label="MCP Servers" value={stats.mcps} />
      <StatItem label="Followers" value={stats.followers} />
      <StatItem label="Total Views" value={stats.totalViews} />
      <StatItem label="Total Copies" value={stats.totalCopies} />
    </div>
  );
}

function StatItem({ label, value }: { label: string; value: number }) {
  return (
    <div className="text-center">
      <p className={cn(mode.typography.headline.md, mode.font, mode.color.text.primary)}>
        {value.toLocaleString()}
      </p>
      <p className={cn(mode.typography.label.md, mode.font, mode.color.text.muted)}>
        {label}
      </p>
    </div>
  );
}
```

## Design System Compliance

### Spacing (8-point grid)
- Hero sections: `space-y-4`
- Stats grid: `gap-6`
- Contributions list: `space-y-4`

### Colors (design tokens only)
```tsx
mode.color.text.primary   // Name, values
mode.color.text.muted     // Labels, descriptions
mode.color.text.warning   // Pinned icon
mode.color.border.default // Section dividers
```

### Typography
```tsx
mode.typography.display.lg    // Username
mode.typography.headline.md   // Stat values
mode.typography.body.md       // Bio, descriptions
mode.typography.label.md      // Stat labels
```

## Checklist for Profile Features

- [ ] ProfileHero displays avatar, name, bio
- [ ] Stats show rules, MCPs, views, copies, followers
- [ ] Social links display when present
- [ ] Follow button works for logged-in users
- [ ] Owner sees edit button instead of follow
- [ ] Contributions list with pinning (owner only)
- [ ] Profile views increment for non-owners
- [ ] Average rating calculated from reviews
- [ ] Leaderboard rank displayed if applicable
- [ ] All spacing follows 8-point grid
- [ ] All colors use design tokens

## File Organization

```
src/app/(directory)/u/[username]/
  ├── page.tsx              # Server - data fetching
  └── components/
      ├── profile-layout.tsx
      ├── profile-hero.tsx
      ├── profile-stats.tsx
      ├── social-links.tsx
      ├── contributions-list.tsx
      └── follow-button.tsx

src/app/(platform)/settings/profile/
  └── page.tsx              # Profile edit form

src/app/api/user/
  ├── profile/route.ts      # GET, PUT profile data
  ├── follow/route.ts       # POST follow/unfollow
  └── pin/route.ts          # POST pin/unpin content
```
