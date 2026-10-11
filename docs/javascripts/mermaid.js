const mermaidScript = document.createElement('script');
mermaidScript.src = 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.min.js';
mermaidScript.onload = () => {
  mermaid.initialize({
    startOnLoad: false,
    theme: 'default',
    themeVariables: {
      primaryColor: '#e65100',
      primaryTextColor: '#fff',
      primaryBorderColor: '#e65100',
      lineColor: '#e65100',
      secondaryColor: '#ffcc80',
      tertiaryColor: '#fff3e0',
      background: 'transparent',
      mainBkg: 'transparent',
      secondBkg: 'transparent',
      tertiaryBkg: 'transparent',
      textColor: '#1d1d1d',
      nodeBorder: '#e65100',
      clusterBorder: '#e65100',
      defaultLinkColor: '#e65100',
      titleColor: '#1d1d1d',
      edgeLabelBackground: '#fff3e0',
      actorBorder: '#e65100',
      actorTextColor: '#1d1d1d',
      actorBkg: '#ffcc80',
      useMaxWidth: true,
    },
    flowchart: {
      useMaxWidth: true,
      htmlLabels: true,
      curve: 'basis',
    },
    sequence: {
      useMaxWidth: true,
      diagramMarginX: 50,
      diagramMarginY: 10,
      actorMargin: 50,
      width: 150,
      height: 65,
      boxMargin: 10,
      boxTextMargin: 5,
      noteMargin: 10,
      messageMargin: 35,
      mirrorActors: true,
      bottomMarginAdj: 1,
    },
    gantt: {
      useMaxWidth: true,
    },
    securityLevel: 'loose',
    fontFamily: 'Inter, system-ui, sans-serif',
    fontSize: 14,
  });
  renderMermaid();
};
document.head.appendChild(mermaidScript);

function renderMermaid() {
  if (typeof mermaid !== 'undefined') {
    mermaid.run({ querySelector: '.mermaid:not([data-processed])' });
  }
}

document.addEventListener('DOMContentLoaded', renderMermaid);

if (typeof document$ !== 'undefined') {
  document$.subscribe(renderMermaid);
}