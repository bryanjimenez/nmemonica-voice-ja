# @nmemonica/voice-ja
Japanese voice module for [Nmemonica](https://bryanjimenez.github.io/nmemonica/)

This repository is part of the Nmemonica project.


## Credits
Thanks to:

 - The following projects for powering Nmemonica's audio/pronunciation:  
    + Software
      - [hts_engine](https://hts-engine.sourceforge.net/)  
Copyright (c) 2001-2015  Nagoya Institute of Technology Department of Computer Science  
2001-2008  Tokyo Institute of Technology Interdisciplinary Graduate School of Science and Engineering  
All rights reserved.  
        <details><summary>LICENSE</summary>

        ```
        /* ----------------------------------------------------------------- */
        /*           The HMM-Based Speech Synthesis Engine "hts_engine API"  */
        /*           developed by HTS Working Group                          */
        /*           http://hts-engine.sourceforge.net/                      */
        /* ----------------------------------------------------------------- */
        /*                                                                   */
        /*  Copyright (c) 2001-2015  Nagoya Institute of Technology          */
        /*                           Department of Computer Science          */
        /*                                                                   */
        /*                2001-2008  Tokyo Institute of Technology           */
        /*                           Interdisciplinary Graduate School of    */
        /*                           Science and Engineering                 */
        /*                                                                   */
        /* All rights reserved.                                              */
        /*                                                                   */
        /* Redistribution and use in source and binary forms, with or        */
        /* without modification, are permitted provided that the following   */
        /* conditions are met:                                               */
        /*                                                                   */
        /* - Redistributions of source code must retain the above copyright  */
        /*   notice, this list of conditions and the following disclaimer.   */
        /* - Redistributions in binary form must reproduce the above         */
        /*   copyright notice, this list of conditions and the following     */
        /*   disclaimer in the documentation and/or other materials provided */
        /*   with the distribution.                                          */
        /* - Neither the name of the HTS working group nor the names of its  */
        /*   contributors may be used to endorse or promote products derived */
        /*   from this software without specific prior written permission.   */
        /*                                                                   */
        /* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND            */
        /* CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,       */
        /* INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF          */
        /* MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE          */
        /* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS */
        /* BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,          */
        /* EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED   */
        /* TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,     */
        /* DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON */
        /* ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,   */
        /* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY    */
        /* OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE           */
        /* POSSIBILITY OF SUCH DAMAGE.                                       */
        /* ----------------------------------------------------------------- */
        ```
        </details>

      - [openjtalk](https://open-jtalk.sourceforge.net/)  
Copyright (c) 2008-2018  Nagoya Institute of Technology Department of Computer Science  
All rights reserved.  
              <details><summary>LICENSE</summary>

        ```
        ===============================================================================
                      The Japanese TTS System "Open JTalk" version 1.11
                                  release December 25, 2018


        The Open JTalk is a Japanese TTS System. It has been being developed by HTS
        working group (see "Who we are" below) and some graduate students in Nagoya
        Institute of Technology (see "AUTHORS" in the same directory).

        *******************************************************************************
                                            Copying
        *******************************************************************************

        The Open JTalk is released under the Modified BSD license (see
        http://www.opensource.org/). Using and distributing this software is free
        (without restriction including without limitation the rights to use, copy,
        modify, merge, publish, distribute, sublicense, and/or sell copies of this
        work, and to permit persons to whom this work is furnished to do so) subject to
        the conditions in the following license:

        /* ----------------------------------------------------------------- */
        /*           The Japanese TTS System "Open JTalk"                    */
        /*           developed by HTS Working Group                          */
        /*           http://open-jtalk.sourceforge.net/                      */
        /* ----------------------------------------------------------------- */
        /*                                                                   */
        /*  Copyright (c) 2008-2018  Nagoya Institute of Technology          */
        /*                           Department of Computer Science          */
        /*                                                                   */
        /* All rights reserved.                                              */
        /*                                                                   */
        /* Redistribution and use in source and binary forms, with or        */
        /* without modification, are permitted provided that the following   */
        /* conditions are met:                                               */
        /*                                                                   */
        /* - Redistributions of source code must retain the above copyright  */
        /*   notice, this list of conditions and the following disclaimer.   */
        /* - Redistributions in binary form must reproduce the above         */
        /*   copyright notice, this list of conditions and the following     */
        /*   disclaimer in the documentation and/or other materials provided */
        /*   with the distribution.                                          */
        /* - Neither the name of the HTS working group nor the names of its  */
        /*   contributors may be used to endorse or promote products derived */
        /*   from this software without specific prior written permission.   */
        /*                                                                   */
        /* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND            */
        /* CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,       */
        /* INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF          */
        /* MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE          */
        /* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS */
        /* BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,          */
        /* EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED   */
        /* TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,     */
        /* DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON */
        /* ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,   */
        /* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY    */
        /* OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE           */
        /* POSSIBILITY OF SUCH DAMAGE.                                       */
        /* ----------------------------------------------------------------- */

        Although this software is free, we still offer no warranties and no
        maintenance. We will continue to endeavor to fix bugs and answer queries when
        can, but are not in a position to guarantee it. We will consider consultancy if
        desired, please contacts us for details.

        If you are using the Open JTalk in commercial environments, even though no
        license is required, we would be grateful if you let us know as it helps
        justify ourselves to our various sponsors. We also strongly encourage you to

        * refer to the use of Open JTalk in any publications that use this software
        * report bugs, where possible with bug fixes, that are found

        See also three "COPYING" files in "mecab", "mecab-naist-jdic", and the current
        directory for details.

        *******************************************************************************
                                        Installation
        *******************************************************************************

        See "INSTALL" in the same directory for details.

        *******************************************************************************
                                      Acknowledgements
        *******************************************************************************

        Keiichi Tokuda
        Keiichiro Oura
        Shinji Sako
        Galatea Project

        *******************************************************************************
                                          Who we are
        *******************************************************************************

        The HTS working group is a voluntary group for developing the HMM-Based Speech
        Synthesis System. Current members are

        Keiichi Tokuda      http://www.sp.nitech.ac.jp/~tokuda/
        (Produce and Design)
        Keiichiro Oura      http://www.sp.nitech.ac.jp/~uratec/
        (Design and Development, Main Maintainer)
        Kei Hashimoto       http://www.sp.nitech.ac.jp/~bonanza/
        Kei Sawada          http://www.sp.nitech.ac.jp/~swdkei/
        Takenori Yoshimura  http://www.sp.nitech.ac.jp/~takenori/
        Shinji Takaki       http://www.sp.nitech.ac.jp/~k-prr44/
        Heiga Zen
        Junichi Yamagishi   http://homepages.inf.ed.ac.uk/jyamagis/
        Tomoki Toda         http://spalab.naist.jp/~tomoki/index_e.html
        Takashi Nose
        Shinji Sako         http://www.mmsp.nitech.ac.jp/~sako/
        Alan W. Black       http://www.cs.cmu.edu/~awb/

        and the members are dynamically changing. The current formal contact address of
        HTS working group and a mailing list for HTS users can be found at
        http://hts.sp.nitech.ac.jp/
        ===============================================================================
        ```
        </details>
      - [jbonsai](https://github.com/jpreprocess/jbonsai)  
        Copyright (c) 2023 by JPreprocess Team.
                    <details><summary>LICENSE</summary>

        ```
                BSD 3-Clause License

        Copyright (c) 2023 by JPreprocess Team.

        Redistribution and use in source and binary forms, with or without
        modification, are permitted provided that the following conditions are met:

        1. Redistributions of source code must retain the above copyright notice, this
          list of conditions and the following disclaimer.

        2. Redistributions in binary form must reproduce the above copyright notice,
          this list of conditions and the following disclaimer in the documentation
          and/or other materials provided with the distribution.

        3. Neither the name of the copyright holder nor the names of its
          contributors may be used to endorse or promote products derived from
          this software without specific prior written permission.

        THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
        AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
        IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
        DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
        FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
        DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
        SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
        CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
        OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
        OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
        ```
        </details>
      - [jpreprocess](https://github.com/jpreprocess/jpreprocess)  
        Copyright (c) 2022 by JPreprocess Team.
                          <details><summary>LICENSE</summary>

        ```
                BSD 3-Clause License

        Copyright (c) 2022 by JPreprocess Team.

        Redistribution and use in source and binary forms, with or without
        modification, are permitted provided that the following conditions are met:

        1. Redistributions of source code must retain the above copyright notice, this
          list of conditions and the following disclaimer.

        2. Redistributions in binary form must reproduce the above copyright notice,
          this list of conditions and the following disclaimer in the documentation
          and/or other materials provided with the distribution.

        3. Neither the name of the copyright holder nor the names of its
          contributors may be used to endorse or promote products derived from
          this software without specific prior written permission.

        THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
        AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
        IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
        DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
        FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
        DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
        SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
        CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
        OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
        OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
        ```
        </details>
    + Voices
      - [hts_voice_nitech_jp_atr503_m001-1.05](https://open-jtalk.sourceforge.net)  
        Copyright (c) 2003-2012  Nagoya Institute of Technology Department of Computer Science  
        2003-2008  Tokyo Institute of Technology Interdisciplinary Graduate School of Science and Engineering  
        Some rights reserved.
                                <details><summary>LICENSE</summary>

        ```
        ===============================================================================
                          HTS Voice "NIT ATR503 M001" version 1.05
                                  release December 25, 2012


        HTS voice trained by using the Nitech Japanese Speech Database "NIT ATR503
        M001" is released as a part of Open JTalk (http://open-jtalk.sourceforge.net/).
        This voice consists of HMMs trained by using HMM-based Speech Synthesis System
        (HTS) version 2.3 alpha (http://hts.sp.nitech.ac.jp/).

        *******************************************************************************
                                            Copying
        *******************************************************************************

        # ----------------------------------------------------------------- #
        #           HTS Voice "NIT ATR503 M001"                             #
        #           released by HTS Working Group                           #
        #           http://open-jtalk.sourceforge.net/                      #
        # ----------------------------------------------------------------- #
        #                                                                   #
        #  Copyright (c) 2003-2012  Nagoya Institute of Technology          #
        #                           Department of Computer Science          #
        #                                                                   #
        #                2003-2008  Tokyo Institute of Technology           #
        #                           Interdisciplinary Graduate School of    #
        #                           Science and Engineering                 #
        #                                                                   #
        # Some rights reserved.                                             #
        #                                                                   #
        # This work is licensed under the Creative Commons Attribution 3.0  #
        # license.                                                          #
        #                                                                   #
        # You are free:                                                     #
        #  * to Share - to copy, distribute and transmit the work           #
        #  * to Remix - to adapt the work                                   #
        # Under the following conditions:                                   #
        #  * Attribution - You must attribute the work in the manner        #
        #    specified by the author or licensor (but not in any way that   #
        #    suggests that they endorse you or your use of the work).       #
        # With the understanding that:                                      #
        #  * Waiver - Any of the above conditions can be waived if you get  #
        #    permission from the copyright holder.                          #
        #  * Public Domain - Where the work or any of its elements is in    #
        #    the public domain under applicable law, that status is in no   #
        #    way affected by the license.                                   #
        #  * Other Rights - In no way are any of the following rights       #
        #    affected by the license:                                       #
        #     - Your fair dealing or fair use rights, or other applicable   #
        #       copyright exceptions and limitations;                       #
        #     - The author's moral rights;                                  #
        #     - Rights other persons may have either in the work itself or  #
        #       in how the work is used, such as publicity or privacy       #
        #       rights.                                                     #
        #  * Notice - For any reuse or distribution, you must make clear to #
        #    others the license terms of this work. The best way to do this #
        #    is with a link to this web page.                               #
        #                                                                   #
        # See http://creativecommons.org/ for details.                      #
        # ----------------------------------------------------------------- #

        See also "COPYING" file in the current directory for details.

        *******************************************************************************
                                        Installation
        *******************************************************************************

        See "INSTALL" in the same directory for details.

        *******************************************************************************
                                      Acknowledgements
        *******************************************************************************

        Keiichi Tokuda
        Shinji Sako
        Heiga Zen
        Keiichiro Oura

        *******************************************************************************
                                          Who we are
        *******************************************************************************

        The HTS working group is a voluntary group for developing the HMM-Based Speech
        Synthesis System. Current members are

        Keiichi Tokuda      http://www.sp.nitech.ac.jp/~tokuda/
        (Produce and Design)
        Keiichiro Oura      http://www.sp.nitech.ac.jp/~uratec/
        (Design and Development, Main Maintainer)
        Kei Hashimoto       http://www.sp.nitech.ac.jp/~bonanza/
        Sayaka Shiota       http://www.sp.nitech.ac.jp/~sayaka/
        Shinji Takaki       http://www.sp.nitech.ac.jp/~k-prr44/
        Heiga Zen
        Junichi Yamagishi   http://homepages.inf.ed.ac.uk/jyamagis/
        Tomoki Toda         http://spalab.naist.jp/~tomoki/index_e.html
        Takashi Nose
        Shinji Sako         http://www.mmsp.nitech.ac.jp/~sako/
        Alan W. Black       http://www.cs.cmu.edu/~awb/

        and the members are dynamically changing. The current formal contact address of
        HTS working group and a mailing list for HTS users can be found at
        http://hts.sp.nitech.ac.jp/
        ===============================================================================
        ```
        </details>
      - [tohoku-f01](https://github.com/icn-lab/htsvoice-tohoku-f01)  
        Copyright(c) 2015 Intelligent Communication Network (Ito-Nose) Laboratory Tohoku University.  
        tohoku-f01 is released by Intelligent Communication Network Laboratory, Tohoku University.  
        Some rights reserved.
                                      <details><summary>LICENSE</summary>

        ```
        ----------------------------------------------------------------------------------
        HTS voice tohoku-f01-{angry,happy,neutral,sad}                                   
        Copyright(c) 2015 Intelligent Communication Network (Ito-Nose) Laboratory
                          Tohoku University.

        tohoku-f01 is released by Intelligent Communication Network Laboratory, Tohoku University.
        Some rights reserved.                                                         

        This work is licensed under the Creative Commons Attribution 4.0 license (CC-BY 4.0).
        ----------------------------------------------------------------------------------
        This is a human-readable summary of (and not a substitute for) the license.

        You are free to:

        Share — copy and redistribute the material in any medium or format
        Adapt — remix, transform, and build upon the material
        for any purpose, even commercially.
        The licensor cannot revoke these freedoms as long as you follow the license terms.


        Under the following terms:

        Attribution — You must give appropriate credit, provide a link to the license, and indicate if changes were made. You may do so in any reasonable manner, but not in any way that suggests the licensor endorses you or your use.
        No additional restrictions — You may not apply legal terms or technological measures that legally restrict others from doing anything the license permits.


        Notices:

        You do not have to comply with the license for elements of the material in the public domain or where your use is permitted by an applicable exception or limitation.
        No warranties are given. The license may not give you all of the permissions necessary for your intended use. For example, other rights such as publicity, privacy, or moral rights may limit how you use the material.
        ```
        </details>
