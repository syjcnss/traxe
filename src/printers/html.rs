use anyhow::Result;
use std::io;

use crate::tree::Node;
use super::{Printer, PrintContext};

pub struct HtmlPrinter {
    tx_hash: String,
    native_symbol: String,
}

impl HtmlPrinter {
    pub fn new(ctx: &PrintContext) -> Self {
        Self {
            tx_hash: ctx.tx_hash.clone(),
            native_symbol: ctx.native_symbol.clone(),
        }
    }
}

impl Printer for HtmlPrinter {
    fn print_to_file(&self) -> bool {
        true
    }

    fn default_path(&self) -> Option<std::path::PathBuf> {
        Some(std::path::PathBuf::from(format!("{}.html", self.tx_hash)))
    }

    fn print(&self, root: &Node, out: &mut dyn io::Write) -> Result<()> {
        let data_json = serde_json::to_string(root)?;
        // Prevent </script> in JSON from breaking the script tag
        let data_json = data_json.replace("</", "<\\/");
        let tx_hash_json = serde_json::to_string(&self.tx_hash)?;
        let native_symbol_json = serde_json::to_string(&self.native_symbol)?;

        let html = TEMPLATE
            .replace("PLACEHOLDER_TX_HASH", &tx_hash_json)
            .replace("PLACEHOLDER_NATIVE_SYMBOL", &native_symbol_json)
            .replace("PLACEHOLDER_TRACE_DATA", &data_json);

        write!(out, "{}", html)?;
        Ok(())
    }
}

const TEMPLATE: &str = r###"<!DOCTYPE html>
<html lang="en" class="dark">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>trace: PLACEHOLDER_TX_HASH</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <script>tailwind.config = { darkMode: 'class' }</script>
  <style>
    ::-webkit-scrollbar { width: 6px; height: 6px; }
    html.dark ::-webkit-scrollbar-track { background: #09090b; }
    html.dark ::-webkit-scrollbar-thumb { background: #27272a; border-radius: 3px; }
    html.dark ::-webkit-scrollbar-thumb:hover { background: #3f3f46; }
    html:not(.dark) ::-webkit-scrollbar-track { background: #fafafa; }
    html:not(.dark) ::-webkit-scrollbar-thumb { background: #d4d4d8; border-radius: 3px; }
    html:not(.dark) ::-webkit-scrollbar-thumb:hover { background: #a1a1aa; }
  </style>
</head>
<body class="bg-white dark:bg-zinc-950">
  <div id="root"></div>

  <script>
    window.__TX_HASH__ = PLACEHOLDER_TX_HASH;
    window.__NATIVE_SYMBOL__ = PLACEHOLDER_NATIVE_SYMBOL;
    window.__TRACE_DATA__ = PLACEHOLDER_TRACE_DATA;
  </script>

  <script src="https://unpkg.com/react@18/umd/react.production.min.js" crossorigin></script>
  <script src="https://unpkg.com/react-dom@18/umd/react-dom.production.min.js" crossorigin></script>
  <script src="https://unpkg.com/@babel/standalone/babel.min.js"></script>

  <script type="text/babel">
    // ── helpers ──────────────────────────────────────────────────────────────

    const CALL_TYPE_CLASSES = {
      CALL:         'bg-cyan-950 text-cyan-300 border-cyan-800',
      DELEGATECALL: 'bg-indigo-950 text-indigo-300 border-indigo-800',
      STATICCALL:   'bg-emerald-950 text-emerald-300 border-emerald-800',
      CALLCODE:     'bg-purple-950 text-purple-300 border-purple-800',
      CREATE:       'bg-orange-950 text-orange-300 border-orange-800',
      CREATE2:      'bg-amber-950 text-amber-200 border-amber-800',
    };

    function parseHexInt(s) {
      if (!s) return 0;
      const h = s.startsWith('0x') || s.startsWith('0X') ? s.slice(2) : s;
      return parseInt(h, 16) || 0;
    }

    function formatValue(value, nativeSymbol) {
      if (!value || value === '0x0' || value === '0') return null;
      try {
        const wei = BigInt(value);
        if (wei > 0n) {
          const eth = Number(wei) / 1e18;
          return `${eth.toFixed(6)} ${nativeSymbol}`;
        }
      } catch {}
      return null;
    }

    function formatGas(gasUsed) {
      const n = parseHexInt(gasUsed);
      return n.toLocaleString();
    }

    function valueClass(v) {
      if (/^0x[0-9a-fA-F]{40}$/.test(v)) return 'text-amber-600 dark:text-amber-300';
      if (v === 'true') return 'text-emerald-600 dark:text-emerald-400';
      if (v === 'false') return 'text-red-600 dark:text-red-400';
      if (v.startsWith('0x')) return 'text-slate-500 dark:text-slate-400';
      if (/^-?\d/.test(v)) return 'text-sky-600 dark:text-sky-300';
      return 'text-zinc-700 dark:text-zinc-200';
    }

    function truncate(s, max = 120) {
      return s.length > max ? s.slice(0, max) + '…' : s;
    }

    function isTransparentDelegate(node) {
      const callChildren = (node.children || []).filter(c => c.type === 'call');
      if (callChildren.length !== 1) return false;
      const delegate = callChildren[0];
      if (delegate.call_type !== 'DELEGATECALL') return false;
      return node.input === delegate.input && node.output === delegate.output;
    }

    // ── small components ─────────────────────────────────────────────────────

    function CallTypeBadge({ callType }) {
      const cls = CALL_TYPE_CLASSES[callType] || 'bg-zinc-900 text-zinc-400 border-zinc-700';
      return (
        <span className={`inline-flex items-center justify-center px-1.5 py-0.5 text-[0.65rem] font-bold font-mono rounded tracking-widest min-w-[88px] text-center border ${cls}`}>
          {callType}
        </span>
      );
    }

    function Chip({ label, className }) {
      return (
        <span className={`inline-flex items-center px-1.5 py-0.5 text-[0.65rem] font-mono rounded border ${className}`}>
          {label}
        </span>
      );
    }

    function ArgRow({ arg }) {
      const { name, ty, value } = arg;
      const hasName = name && name !== '_';
      const vc = valueClass(value);
      return (
        <div className="flex gap-4 items-start py-0.5 pl-1">
          <div className="min-w-[160px] shrink-0">
            {hasName && (
              <span className="font-mono text-xs text-zinc-600 dark:text-zinc-300 font-medium">{name}</span>
            )}
            <span className="font-mono text-[0.7rem] text-zinc-400 dark:text-zinc-600">
              {hasName ? ` (${ty})` : `(${ty})`}
            </span>
          </div>
          <div className={`font-mono text-xs break-all ${vc}`}>
            {truncate(value)}
          </div>
        </div>
      );
    }

    function ArgsSection({ label, args }) {
      if (!args || args.length === 0) return null;
      return (
        <div className="my-1.5 pl-3 border-l-2 border-zinc-200 dark:border-zinc-800">
          <p className="text-[0.65rem] text-zinc-400 dark:text-zinc-600 italic mb-1 font-mono">{label}</p>
          {args.map((arg, i) => <ArgRow key={i} arg={arg} />)}
        </div>
      );
    }

    function RawDataSection({ label, hex }) {
      if (!hex) return null;
      return (
        <div className="my-1.5 pl-3 border-l-2 border-zinc-200 dark:border-zinc-800">
          <p className="text-[0.65rem] text-zinc-400 dark:text-zinc-600 italic mb-1 font-mono">{label}</p>
          <p className="font-mono text-[0.65rem] text-zinc-500 dark:text-zinc-700 break-all">{hex}</p>
        </div>
      );
    }

    // ── EventNodeRow ─────────────────────────────────────────────────────────

    function EventNodeRow({ event }) {
      const [open, setOpen] = React.useState(true);
      const hasDecoded   = event.decoded_args && event.decoded_args.length > 0;
      const hasRawTopics = event.topics && event.topics.length > 1;
      const hasRawData   = event.data && event.data !== '0x';
      const hasContent   = hasDecoded || hasRawTopics || hasRawData;

      const label = event.event_name
        || (event.topics && event.topics[0] ? `0x${event.topics[0].replace(/^0x/, '')}` : '(unknown)');

      return (
        <div className="mt-0.5">
          <div
            className={`flex items-center flex-wrap gap-1.5 px-1 py-1 rounded select-none transition-colors${hasContent ? ' cursor-pointer hover:bg-zinc-100 dark:hover:bg-zinc-900' : ''}`}
            onClick={() => hasContent && setOpen(o => !o)}
          >
            <span className="inline-flex items-center justify-center px-1.5 py-0.5 min-w-[88px] text-[0.65rem] font-bold font-mono rounded tracking-widest border bg-yellow-950 text-yellow-300 border-yellow-800">
              EVENT
            </span>
            <span className="text-zinc-400 dark:text-zinc-700">·</span>
            <span className={`font-mono text-sm ${event.event_name ? 'text-emerald-600 dark:text-emerald-400 font-semibold' : 'text-zinc-500'}`}>
              {label}
            </span>
            <span className="font-mono text-xs text-zinc-500 dark:text-zinc-600">{event.address}</span>
          </div>

          {hasContent && open && (
            <div className="pl-4 ml-1 border-l border-zinc-200 dark:border-zinc-800">
              {hasDecoded
                ? <ArgsSection label="args" args={event.decoded_args} />
                : <>
                    {(event.topics || []).slice(1).map((t, i) => (
                      <RawDataSection key={i} label={`topic${i + 1}`} hex={t} />
                    ))}
                    {hasRawData && <RawDataSection label="data" hex={event.data} />}
                  </>
              }
            </div>
          )}
        </div>
      );
    }

    // ── CallNodeRow ───────────────────────────────────────────────────────────

    function CallNodeRow({ node, isRoot, showRawData, showEvents, showGas, nativeSymbol }) {
      const [open, setOpen] = React.useState(true);

      const input    = node.input || '';
      const hexInput = input.replace(/^0x/i, '');
      const selector = hexInput.length >= 8 ? `0x${hexInput.slice(0, 8)}` : null;
      const valueStr = formatValue(node.value, nativeSymbol);
      const gasStr   = formatGas(node.gas_used);

      const targetLabel = node.contract_label || node.to || node.from;
      const showAddr    = !!(node.contract_label && node.to);

      const skipIO        = isTransparentDelegate(node);
      const hasDecodedIn  = !skipIO && node.decoded_input  && node.decoded_input.length  > 0;
      const hasDecodedOut = !skipIO && node.decoded_output && node.decoded_output.length > 0;
      const hasRawIn      = !skipIO && showRawData;
      const hasRawOut     = !skipIO && showRawData;
      const rawIn         = (input && input !== '') ? input : '0x';
      const rawOut        = (node.output && node.output !== '') ? node.output : '0x';

      const visibleChildren = (node.children || []).filter(c =>
        c.type === 'call' || (showEvents && c.type === 'event')
      );
      const hasContent = hasDecodedIn || hasDecodedOut || hasRawIn || hasRawOut || visibleChildren.length > 0;

      return (
        <div className="mt-0.5">
          {/* header row */}
          <div
            className={`flex items-center flex-wrap gap-1.5 px-1 py-1 rounded select-none transition-colors${hasContent ? ' cursor-pointer hover:bg-zinc-100 dark:hover:bg-zinc-900' : ''}`}
            onClick={() => hasContent && setOpen(o => !o)}
          >
            {hasContent ? (
              <span className={`text-[0.55rem] text-zinc-400 dark:text-zinc-600 w-2.5 shrink-0 inline-block transition-transform duration-150${open ? ' rotate-90' : ''}`}>▶</span>
            ) : (
              <span className="w-2.5 shrink-0" />
            )}

            <CallTypeBadge callType={node.call_type} />

            <span className="font-mono font-semibold text-sm text-zinc-800 dark:text-zinc-200">
              {targetLabel}
            </span>
            {showAddr && (
              <span className="font-mono text-xs text-zinc-500 dark:text-zinc-600">({node.to})</span>
            )}

            {(node.function_name || selector) && (
              <span className="text-zinc-400 dark:text-zinc-700">·</span>
            )}
            {node.function_name && (
              <span className="font-mono font-bold text-sm text-emerald-600 dark:text-emerald-400">
                {node.function_name}
              </span>
            )}
            {selector && (
              <span className={`font-mono text-xs ${node.function_name ? 'text-zinc-500 dark:text-zinc-600' : 'text-emerald-600 dark:text-emerald-400 font-bold'}`}>
                {node.function_name ? `[${selector}]` : selector}
              </span>
            )}

            {valueStr && <Chip label={valueStr} className="text-violet-400 border-violet-800/50 bg-violet-950/40" />}
            {showGas && <Chip label={`${gasStr} gas`} className="text-zinc-500 dark:text-zinc-600 border-zinc-300 dark:border-zinc-800 bg-transparent" />}
            {node.error && <Chip label={`revert: ${node.error}`} className="text-red-400 border-red-900/60 bg-red-950/30" />}
          </div>

          {/* body */}
          {hasContent && open && (
            <div className="pl-4 ml-2.5 border-l border-zinc-200 dark:border-zinc-800">
              {hasRawIn      && <RawDataSection label="raw input"      hex={rawIn} />}
              {hasDecodedIn  && <ArgsSection    label="decoded input"  args={node.decoded_input} />}
              {hasRawOut     && <RawDataSection label="raw output"     hex={rawOut} />}
              {hasDecodedOut && <ArgsSection    label="decoded output" args={node.decoded_output} />}

              {visibleChildren.map((child, i) =>
                child.type === 'call'
                  ? <CallNodeRow key={i} node={child} isRoot={false} showRawData={showRawData} showEvents={showEvents} showGas={showGas} nativeSymbol={nativeSymbol} />
                  : <EventNodeRow key={i} event={child} />
              )}
            </div>
          )}
        </div>
      );
    }

    // ── Toggle ────────────────────────────────────────────────────────────────

    function Toggle({ checked, onChange, label }) {
      return (
        <label className="flex items-center gap-1.5 cursor-pointer shrink-0">
          <button
            role="switch"
            aria-checked={checked}
            onClick={() => onChange(!checked)}
            className={`relative inline-flex h-5 w-9 shrink-0 items-center rounded-full border transition-colors duration-200 ${checked ? 'bg-violet-600 border-violet-500' : 'bg-zinc-200 border-zinc-300 dark:bg-zinc-800 dark:border-zinc-700'}`}
          >
            <span className={`inline-block h-3.5 w-3.5 transform rounded-full bg-white shadow-sm transition-transform duration-200 ${checked ? 'translate-x-[18px]' : 'translate-x-0.5'}`} />
          </button>
          <span className="text-xs text-zinc-500 dark:text-zinc-400">{label}</span>
        </label>
      );
    }

    // ── App ───────────────────────────────────────────────────────────────────

    function App() {
      const [showRawData, setShowRawData] = React.useState(false);
      const [showEvents,  setShowEvents]  = React.useState(true);
      const [showGas,     setShowGas]     = React.useState(true);
      const [isDark,      setIsDark]      = React.useState(true);

      React.useEffect(() => {
        if (isDark) {
          document.documentElement.classList.add('dark');
        } else {
          document.documentElement.classList.remove('dark');
        }
      }, [isDark]);

      const txHash       = window.__TX_HASH__;
      const nativeSymbol = window.__NATIVE_SYMBOL__;
      const traceData    = window.__TRACE_DATA__;

      const senderAddr = traceData && traceData.type === 'call' ? traceData.from : null;

      return (
        <div className="min-h-screen bg-white dark:bg-zinc-950">
          {/* AppBar */}
          <header className="fixed top-0 left-0 right-0 z-50 bg-zinc-100 border-b border-zinc-200 dark:bg-zinc-900 dark:border-zinc-800">
            <div className="flex items-center gap-3 px-4 h-12">
              <span className="font-bold text-sm text-violet-400 tracking-tight shrink-0">
                traxe
              </span>
              <div className="w-px h-4 bg-zinc-300 dark:bg-zinc-700 shrink-0" />
              <span
                title={txHash}
                className="font-mono text-xs text-zinc-500 flex-1 overflow-hidden text-ellipsis whitespace-nowrap cursor-default"
              >
                {txHash}
              </span>
              <div className="flex items-center gap-4 shrink-0">
                <Toggle checked={showRawData} onChange={setShowRawData} label="Raw" />
                <Toggle checked={showEvents}  onChange={setShowEvents}  label="Events" />
                <Toggle checked={showGas}     onChange={setShowGas}     label="Gas" />
                <Toggle checked={isDark}      onChange={setIsDark}      label="Dark" />
              </div>
            </div>
          </header>

          {/* Content */}
          <div className="pt-14 px-3 sm:px-5 md:px-7 pb-24">
            {senderAddr && (
              <p className="font-mono text-xs text-zinc-500 mt-5 mb-1">
                from <span className="text-amber-600 dark:text-amber-300">{senderAddr}</span>
              </p>
            )}
            {traceData && traceData.type === 'call' && (
              <CallNodeRow
                node={traceData}
                isRoot={true}
                showRawData={showRawData}
                showEvents={showEvents}
                showGas={showGas}
                nativeSymbol={nativeSymbol}
              />
            )}
          </div>
        </div>
      );
    }

    ReactDOM.createRoot(document.getElementById('root')).render(<App />);
  </script>
</body>
</html>
"###;
