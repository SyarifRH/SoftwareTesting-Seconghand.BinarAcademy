<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>section_Nama                            Kot_ff7c90</name>
   <tag></tag>
   <elementGuidId>cdb22ce3-74f7-4a5a-a423-979996f1133e</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>section.pt-5.mt-5</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Lengkapi Info Akun'])[1]/following::section[1]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>section</value>
      <webElementGuid>e47f33f7-1a06-43e4-bcaa-f513215eacb1</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>pt-5 mt-5</value>
      <webElementGuid>3e962d7b-8cec-4ffd-9cd0-8acc859b0b73</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
      
  
  
    
  


  
  
    

      
        
            
              
            

          
        
      

      
        Nama
        
      

      
        Kota
        Pilih Kota
Solo
Jogja
Jakarta
Bandung
Semarang
      

      
        Alamat
        Jl. Jalan II
      

      
        No Handphone
        
      

      
        
      
  



  var formAvatarView = document.getElementById('form-avatar-view')
  var formAvatarInput = document.getElementById('form-avatar-input')
  var formAvatar = document.getElementById('form-avatar')
  var formAvatarImage = document.getElementById('form-avatar-image')
  var image;

  formAvatarView.addEventListener('click', function(e) {
    formAvatar.click();
  });

  formAvatar.addEventListener('change', function () {
    if (!!formAvatarImage) {
      formAvatarImage.style[&quot;display&quot;] = &quot;none&quot;;
    }

    if (!!image) {
      image.remove();
    }

    for (const file of formAvatar.files) {
      image = document.createElement('img');
      image.classList.add(&quot;p-0&quot;, &quot;img-thumbnail&quot;, &quot;img-avatar-profile&quot;, &quot;overflow-hidden&quot;, &quot;rounded-4&quot;, &quot;border-0&quot;);
      image.src = URL.createObjectURL(file);
      formAvatarView.prepend(image);
      formAvatarInput.style['display'] = 'none';
    }
  });




    </value>
      <webElementGuid>f97b6c23-622c-4ef3-b0b8-213b6bb7e10a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]/section[@class=&quot;pt-5 mt-5&quot;]</value>
      <webElementGuid>81af934d-ebd0-405a-bbea-797d90f203ca</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Lengkapi Info Akun'])[1]/following::section[1]</value>
      <webElementGuid>7d16dc32-2db1-4995-9234-4b36873faecd</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//section</value>
      <webElementGuid>b28aa15c-f7ff-496a-9e80-60516a88e9d1</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//section[(text() = concat(&quot;
      
  
  
    
  


  
  
    

      
        
            
              
            

          
        
      

      
        Nama
        
      

      
        Kota
        Pilih Kota
Solo
Jogja
Jakarta
Bandung
Semarang
      

      
        Alamat
        Jl. Jalan II
      

      
        No Handphone
        
      

      
        
      
  



  var formAvatarView = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-view&quot; , &quot;'&quot; , &quot;)
  var formAvatarInput = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-input&quot; , &quot;'&quot; , &quot;)
  var formAvatar = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar&quot; , &quot;'&quot; , &quot;)
  var formAvatarImage = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-image&quot; , &quot;'&quot; , &quot;)
  var image;

  formAvatarView.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
    formAvatar.click();
  });

  formAvatar.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function () {
    if (!!formAvatarImage) {
      formAvatarImage.style[&quot;display&quot;] = &quot;none&quot;;
    }

    if (!!image) {
      image.remove();
    }

    for (const file of formAvatar.files) {
      image = document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
      image.classList.add(&quot;p-0&quot;, &quot;img-thumbnail&quot;, &quot;img-avatar-profile&quot;, &quot;overflow-hidden&quot;, &quot;rounded-4&quot;, &quot;border-0&quot;);
      image.src = URL.createObjectURL(file);
      formAvatarView.prepend(image);
      formAvatarInput.style[&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;] = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
    }
  });




    &quot;) or . = concat(&quot;
      
  
  
    
  


  
  
    

      
        
            
              
            

          
        
      

      
        Nama
        
      

      
        Kota
        Pilih Kota
Solo
Jogja
Jakarta
Bandung
Semarang
      

      
        Alamat
        Jl. Jalan II
      

      
        No Handphone
        
      

      
        
      
  



  var formAvatarView = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-view&quot; , &quot;'&quot; , &quot;)
  var formAvatarInput = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-input&quot; , &quot;'&quot; , &quot;)
  var formAvatar = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar&quot; , &quot;'&quot; , &quot;)
  var formAvatarImage = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-image&quot; , &quot;'&quot; , &quot;)
  var image;

  formAvatarView.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
    formAvatar.click();
  });

  formAvatar.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function () {
    if (!!formAvatarImage) {
      formAvatarImage.style[&quot;display&quot;] = &quot;none&quot;;
    }

    if (!!image) {
      image.remove();
    }

    for (const file of formAvatar.files) {
      image = document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
      image.classList.add(&quot;p-0&quot;, &quot;img-thumbnail&quot;, &quot;img-avatar-profile&quot;, &quot;overflow-hidden&quot;, &quot;rounded-4&quot;, &quot;border-0&quot;);
      image.src = URL.createObjectURL(file);
      formAvatarView.prepend(image);
      formAvatarInput.style[&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;] = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
    }
  });




    &quot;))]</value>
      <webElementGuid>c5be1480-9dab-436d-b545-cc082e215dd6</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
