<h1><a name="imports">World imports</a></h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi:random_random_0.2.0"><code>wasi:random/random@0.2.0</code></a></li>
<li>interface <a href="#wasi:random_insecure_0.2.0"><code>wasi:random/insecure@0.2.0</code></a></li>
<li>interface <a href="#wasi:random_insecure_seed_0.2.0"><code>wasi:random/insecure-seed@0.2.0</code></a></li>
</ul>
</li>
</ul>
<h2><a name="wasi:random_random_0.2.0">Import interface wasi:random/random@0.2.0</a></h2>
<p>WASI Random is a random data API.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<hr />
<h3>Functions</h3>
<h4><a name="get_random_bytes"><code>get-random-bytes: func</code></a></h4>
<p>Return <code>len</code> cryptographically-secure random or pseudo-random bytes.</p>
<p>This function must produce data at least as cryptographically secure and
fast as an adequately seeded cryptographically-secure pseudo-random
number generator (CSPRNG). It must not block, from the perspective of
the calling program, under any circumstances, including on the first
request and on requests for numbers of bytes. The returned data must
always be unpredictable.</p>
<p>This function must always return fresh data. Deterministic environments
must omit this function, rather than implementing it with deterministic
data.</p>
<h5>Params</h5>
<ul>
<li><a name="get_random_bytes.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="get_random_bytes.0"></a> list&lt;<code>u8</code>&gt;</li>
</ul>
<h4><a name="get_random_u64"><code>get-random-u64: func</code></a></h4>
<p>Return a cryptographically-secure random or pseudo-random <code>u64</code> value.</p>
<p>This function returns the same type of data as <a href="#get_random_bytes"><code>get-random-bytes</code></a>,
represented as a <code>u64</code>.</p>
<h5>Return values</h5>
<ul>
<li><a name="get_random_u64.0"></a> <code>u64</code></li>
</ul>
<h2><a name="wasi:random_insecure_0.2.0">Import interface wasi:random/insecure@0.2.0</a></h2>
<p>The insecure interface for insecure pseudo-random numbers.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<hr />
<h3>Functions</h3>
<h4><a name="get_insecure_random_bytes"><code>get-insecure-random-bytes: func</code></a></h4>
<p>Return <code>len</code> insecure pseudo-random bytes.</p>
<p>This function is not cryptographically secure. Do not use it for
anything related to security.</p>
<p>There are no requirements on the values of the returned bytes, however
implementations are encouraged to return evenly distributed values with
a long period.</p>
<h5>Params</h5>
<ul>
<li><a name="get_insecure_random_bytes.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="get_insecure_random_bytes.0"></a> list&lt;<code>u8</code>&gt;</li>
</ul>
<h4><a name="get_insecure_random_u64"><code>get-insecure-random-u64: func</code></a></h4>
<p>Return an insecure pseudo-random <code>u64</code> value.</p>
<p>This function returns the same type of pseudo-random data as
<a href="#get_insecure_random_bytes"><code>get-insecure-random-bytes</code></a>, represented as a <code>u64</code>.</p>
<h5>Return values</h5>
<ul>
<li><a name="get_insecure_random_u64.0"></a> <code>u64</code></li>
</ul>
<h2><a name="wasi:random_insecure_seed_0.2.0">Import interface wasi:random/insecure-seed@0.2.0</a></h2>
<p>The insecure-seed interface for seeding hash-map DoS resistance.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<hr />
<h3>Functions</h3>
<h4><a name="insecure_seed"><code>insecure-seed: func</code></a></h4>
<p>Return a 128-bit value that may contain a pseudo-random value.</p>
<p>The returned value is not required to be computed from a CSPRNG, and may
even be entirely deterministic. Host implementations are encouraged to
provide pseudo-random values to any program exposed to
attacker-controlled content, to enable DoS protection built into many
languages' hash-map implementations.</p>
<p>This function is intended to only be called once, by a source language
to initialize Denial Of Service (DoS) protection in its hash-map
implementation.</p>
<h1>Expected future evolution</h1>
<p>This will likely be changed to a value import, to prevent it from being
called multiple times and potentially used for purposes other than DoS
protection.</p>
<h5>Return values</h5>
<ul>
<li><a name="insecure_seed.0"></a> (<code>u64</code>, <code>u64</code>)</li>
</ul>
