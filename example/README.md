# CryEngine example

## Workshop requirements

- The `appid` must reference a game within your [Steam library](https://store.steampowered.com) that you have [permissions to access](https://steamcommunity.com/sharedfiles/filedetails/?id=183087266).
- The `fileid` will be returned upon the first successful workshop upload request.  Subsequent requests **will require this value**.
- Both `changenote` and `tags` are optional.

```xml
// Workshop/NewProject/config.xml

<config>
    <appid></appid>
    <title>NewProject</title>
    <description>This is my NewProject.</description>
    <changenote></changenote>
    <tags></tags>
    <fileid></fileid>
</config>
```

## Upload [CryEngine](https://www.cryengine.com) game mod to Steam

From within this directory run the following command:

    $ ./steam-workshop-bundler --username <USERNAME> --password <PASSWORD> --guard-code <GUARD_CODE> --workshop NewProject
