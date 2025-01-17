<!------------------------------------------------------------------------------
  This file is a part of the "Lady Deirdre" work,
  a compiler front-end foundation technology.

  This work is proprietary software with source-available code.

  To copy, use, distribute, and contribute to this work, you must agree to
  the terms of the General License Agreement:

  https://github.com/Eliah-Lakhin/lady-deirdre/blob/master/EULA.md.

  The agreement grants you a Commercial-Limited License that gives you
  the right to use my work in non-commercial and limited commercial products
  with a total gross revenue cap. To remove this commercial limit for one of
  your products, you must acquire an Unrestricted Commercial License.

  If you contribute to the source code, documentation, or related materials
  of this work, you must assign these changes to me. Contributions are
  governed by the "Derivative Work" section of the General License
  Agreement.

  Copying the work in parts is strictly forbidden, except as permitted under
  the terms of the General License Agreement.

  If you do not or cannot agree to the terms of this Agreement,
  do not use this work.

  This work is provided "as is" without any warranties, express or implied,
  except to the extent that such disclaimers are held to be legally invalid.

  Copyright (c) 2024 Ilya Lakhin (Илья Александрович Лахин).
  All rights reserved.
------------------------------------------------------------------------------->

# Source Code

Lady Deirdre distinguishes between the scanner's implementation, carried out by
the [Token](https://docs.rs/lady-deirdre/2.0.0/lady_deirdre/lexis/trait.Token.html)
type, and the source code manager, responsible for actual code storage and
lexical scanning through interaction with the scanner's implementation.

Each source code manager implements
a [SourceCode](https://docs.rs/lady-deirdre/2.0.0/lady_deirdre/lexis/trait.SourceCode.html)
trait, providing a minimal set of features enabling inspection of its content,
including the source code text and its lexical structure.

The crate offers specialized source code managers with distinct API options and
performance characteristics. For instance, while
the [TokenBuffer](https://docs.rs/lady-deirdre/2.0.0/lady_deirdre/lexis/struct.TokenBuffer.html)
lacks general incremental rescanning capabilities, it offers slightly better
performance than the
mutable [Document](https://docs.rs/lady-deirdre/2.0.0/lady_deirdre/units/enum.Document.html).
