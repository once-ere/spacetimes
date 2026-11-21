(* gui_start.wl -- the first of the three calls that evaluate the notebook superfluid.nb in the front end of
   Wolfram (folder Revision_HS_CJT_and_Kohn-Sham of PROJECT spacetimes).  It is evaluated in the kernel of the running
   Wolfram.app session (a request "gui <this file>" of the watcher on the author's computer).  The work folder
   .git-transfer/mma/sfnb of the author's copy of the repository holds the built notebook in/superfluid.nb, these
   files and the outputs out/superfluid.*; guirun.wl defines SFNBGUI`start, SFNBGUI`go and SFNBGUI`status. *)
Get["/Users/nsh/Developer/github/spacetimes/.git-transfer/mma/sfnb/guirun.wl"];
SFNBGUI`start["/Users/nsh/Developer/github/spacetimes/.git-transfer/mma/sfnb/in/superfluid.nb", "superfluid"]
