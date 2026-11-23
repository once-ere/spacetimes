(* gui_start.wl -- the first of the three calls that evaluate the notebook lord_gravity.nb in the front end of
   Wolfram (folder Revision_Lord_Gravity of PROJECT spacetimes).  It is evaluated in the kernel of the running
   Wolfram.app session (a request "gui <this file>" of the watcher on the author's computer).  The work folder
   .git-transfer/mma/lgnb of the author's copy of the repository holds the built notebook in/lord_gravity.nb, these
   files and the outputs out/lord_gravity.*; guirun.wl defines LGNBGUI`start, LGNBGUI`go and LGNBGUI`status. *)
Get["/Users/nsh/Developer/github/spacetimes/.git-transfer/mma/lgnb/guirun.wl"];
LGNBGUI`start["/Users/nsh/Developer/github/spacetimes/.git-transfer/mma/lgnb/in/lord_gravity.nb", "lord_gravity"]
