lightweight lighting remote controller⚡️

Good Bye モラサーニー・カチョエテ・ウハス3 ...

### Specification

very simple.

- Controll my room lighting remotely on my phone application.

    - room lighting is controlled in python script in my raspberry pi.

```
 +------------------------------------ raspberry pi --------------------------------+
 |                                                                                  |
 | +-------------+                       +---------------+                          |
 | |             |  send event message   |               |                          |
 | |             | --------------------> |               |                          |
 | | rust server |                       | python script |  ====> controll lighting |
 | |             | response event result |               |                          |
 | |             | <-------------------- |               |                          |
 | +-------------+                       +---------------+                          |
 |                                                                                  |
 +----------------------------------------------------------------------------------+
```
