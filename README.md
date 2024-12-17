# bsky-earmuffs

Create & update automatic moderation lists

To make a new list, create a configuration file (see `earmuffs.json` for an example) that names the list and defines includes (accounts to add to the list) and excludes (accounts that will not be included on the list, handled after the includes are resolved).

## example config

Here's an example that uses all the types of sources (followers, follows, explicit list. DIDs can be used in place of handles, if you have them handy.

```json
{
  "auth": {
    "handle": "earmuffs.gnmerritt.net", // this is the account where the list will live
    "app_password": "can be here" // or in an environment variable so you don't leak it
  },
  "lists": [
    {
      "name": "All followers of Example Tim",
      "includes": [
          {"followers_of": "tim.example.com"} // everyone who follows Tim
      ],
      "excludes": [
          {"followed_by": "gnmerritt.net"}, // but not people I follow explicitly
          {"users": ["pleasant.example.com", "fred.example.com"]} // and not these users either
      ]
    }
  ]
}
```

## Contributions

PRs and Issues welcome, any submissions will be licensed the same as the rest of the repository